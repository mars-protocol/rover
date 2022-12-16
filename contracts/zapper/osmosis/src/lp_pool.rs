use cosmwasm_std::{
    attr, to_binary, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Uint128,
};
use cw_asset::AssetList;
use cw_dex::osmosis::OsmosisPool;
use cw_dex::traits::Pool;
use cw_dex::CwDexError;
use mars_zapper_base::{CallbackMsg, ContractError, LpPool};
use std::str::FromStr;

pub struct OsmosisLpPool {}

impl OsmosisLpPool {
    /// Returns the matching pool given a LP token.
    ///
    /// Based on impl from https://github.com/apollodao/cw-dex/blob/develop/src/implementations/pool.rs#L60
    pub fn get_pool_for_lp_token(
        deps: Deps,
        lp_token_denom: &str,
    ) -> Result<OsmosisPool, CwDexError> {
        // The only Pool implementation that uses native denoms right now is Osmosis
        if !lp_token_denom.starts_with("gamm/pool/") {
            return Err(CwDexError::NotLpToken {});
        }

        let pool_id_str = lp_token_denom
            .strip_prefix("gamm/pool/")
            .ok_or(CwDexError::NotLpToken {})?;

        let pool_id = u64::from_str(pool_id_str).map_err(|_| CwDexError::NotLpToken {})?;

        Ok(OsmosisPool::new(pool_id, deps)?)
    }
}

impl LpPool for OsmosisLpPool {
    fn get_pool_for_lp_token(
        deps: Deps,
        lp_token_denom: &str,
    ) -> Result<Box<dyn Pool>, CwDexError> {
        Self::get_pool_for_lp_token(deps, lp_token_denom).map(|p| {
            let as_trait: Box<dyn Pool> = Box::new(p);
            as_trait
        })
    }

    fn execute_provide_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        lp_token_out: String,
        recipient: Option<String>,
        minimum_receive: Uint128,
    ) -> Result<Response, ContractError> {
        let pool = Self::get_pool_for_lp_token(deps.as_ref(), &lp_token_out)?;
        let mut assets: AssetList = info.funds.clone().into();

        // Unwrap recipient or use caller's address
        let recipient = recipient.map_or(Ok(info.sender), |x| deps.api.addr_validate(&x))?;

        let mut event_attrs = vec![attr("assets", assets.to_string())];

        let response = if assets.len() == 1 {
            event_attrs.push(attr("action", "single_sided_provide_liquidity"));

            // Provide single sided
            pool.provide_liquidity(deps.as_ref(), &env, assets.clone(), minimum_receive)?
        } else {
            event_attrs.push(attr("action", "double_sided_provide_liquidity"));

            // Provide as much as possible double sided, and then issue callbacks to
            // provide the remainder single sided
            let (lp_tokens_received, tokens_used) =
                pool.simulate_noswap_join(&deps.querier, &assets)?;

            // Get response with msg to provide double sided
            let mut provide_res =
                pool.provide_liquidity(deps.as_ref(), &env, assets.clone(), lp_tokens_received)?;

            // Deduct tokens used to get remaining tokens
            assets.deduct_many(&tokens_used)?;

            // For each of the remaining tokens, issue a callback to provide
            // liquidity single sided. These must be done as a callbacks, because
            // the simulation inside pool.provide_liquidity will use the current
            // reserves, which will be altered by each of the single sided joins,
            // so the simulations will be incorrect unless we do them one at a time.
            for asset in assets.to_vec() {
                if asset.amount > Uint128::zero() {
                    let msg = CallbackMsg::SingleSidedJoin {
                        lp_token_out: lp_token_out.clone(),
                        coin_in: asset.try_into()?,
                    }
                    .into_cosmos_msg(&env)?;
                    provide_res = provide_res.add_message(msg);
                }
            }

            provide_res
        };

        // Query current contract LP token balance
        let lp_token_balance = deps
            .querier
            .query_balance(&env.contract.address, &lp_token_out)?;

        // Callback to return LP tokens
        let callback_msg = CallbackMsg::ReturnLpTokens {
            balance_before: lp_token_balance,
            recipient,
            minimum_receive,
        }
        .into_cosmos_msg(&env)?;

        let event =
            Event::new("rover/zapper/execute_provide_liquidity").add_attributes(event_attrs);
        Ok(response.add_message(callback_msg).add_event(event))
    }

    /// CallbackMsg handler to provide liquidity with the given assets. This needs
    /// to be a callback, rather than doing in the first ExecuteMsg, because
    /// pool.provide_liquidity does a simulation with current reserves, and our
    /// actions in the top level ExecuteMsg will alter the reserves, which means the
    /// reserves would be wrong in the provide liquidity simulation.
    fn execute_callback_single_sided_join(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        lp_token_out: String,
        coin_in: Coin,
    ) -> Result<Response, ContractError> {
        let pool = Self::get_pool_for_lp_token(deps.as_ref(), &lp_token_out)?;
        let res = pool.provide_liquidity(
            deps.as_ref(),
            &env,
            vec![coin_in.clone()].into(),
            Uint128::one(),
        )?;

        let event = Event::new("rover/zapper/execute_callback_single_sided_join")
            .add_attribute("coin_in", coin_in.to_string());

        Ok(res.add_event(event))
    }

    fn query_estimate_provide_liquidity(
        deps: Deps,
        env: Env,
        lp_token_out: String,
        coins_in: Vec<Coin>,
    ) -> StdResult<Binary> {
        let pool = Self::get_pool_for_lp_token(deps, &lp_token_out)?;

        let mut assets: AssetList = coins_in.into();

        let lp_tokens_returned = if assets.len() == 1 {
            pool.simulate_provide_liquidity(deps, &env, assets)?.amount
        } else {
            let (mut lp_tokens_received, tokens_used) =
                pool.simulate_noswap_join(&deps.querier, &assets)?;

            // Deduct tokens used to get remaining tokens
            assets.deduct_many(&tokens_used)?;

            for asset in assets.to_vec() {
                if asset.amount > Uint128::zero() {
                    let assets = AssetList::from(vec![asset.clone()]);
                    let returned = pool.simulate_provide_liquidity(deps, &env, assets)?;
                    lp_tokens_received += returned.amount;
                }
            }

            lp_tokens_received
        };

        to_binary(&lp_tokens_returned)
    }
}
