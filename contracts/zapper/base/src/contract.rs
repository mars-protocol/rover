use std::marker::PhantomData;

use cosmwasm_std::{
    attr, to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Response,
    StdResult, Uint128,
};
use cw_asset::{Asset, AssetInfo, AssetList};
use cw_utils::one_coin;

use crate::{CallbackMsg, ContractError, ExecuteMsg, InstantiateMsg, LpPool, QueryMsg};

pub struct ZapperBase<P>
where
    P: LpPool,
{
    /// Phantom data holds generics
    pub custom_pool: PhantomData<P>,
}

impl<P> Default for ZapperBase<P>
where
    P: LpPool,
{
    fn default() -> Self {
        Self {
            custom_pool: PhantomData,
        }
    }
}

impl<P> ZapperBase<P>
where
    P: LpPool,
{
    pub fn instantiate(
        &self,
        _deps: DepsMut,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        Ok(Response::default())
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::ProvideLiquidity {
                lp_token_out,
                recipient,
                minimum_receive,
            } => Self::execute_provide_liquidity(
                deps,
                env,
                info,
                lp_token_out,
                recipient,
                minimum_receive,
            ),
            ExecuteMsg::WithdrawLiquidity { recipient } => {
                Self::execute_withdraw_liquidity(deps, env, info, recipient)
            }
            ExecuteMsg::Callback(msg) => {
                // Can only be called by the contract itself
                if info.sender != env.contract.address {
                    return Err(ContractError::Unauthorized {});
                }
                match msg {
                    CallbackMsg::SingleSidedJoin { asset, lp_token } => {
                        Self::execute_callback_single_sided_join(deps, env, info, asset, lp_token)
                    }
                    CallbackMsg::ReturnLpTokens {
                        balance_before,
                        recipient,
                        minimum_receive,
                    } => Self::execute_return_tokens(
                        deps,
                        env,
                        info,
                        balance_before,
                        recipient,
                        minimum_receive,
                    ),
                }
            }
        }
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::EstimateProvideLiquidity {
                lp_token_out,
                coins_in,
            } => Self::query_estimate_provide_liquidity(deps, env, lp_token_out, coins_in),
            QueryMsg::EstimateWithdrawLiquidity { coin_in } => {
                Self::query_estimate_withdraw_liquidity(deps, env, coin_in)
            }
        }
    }

    fn execute_provide_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        lp_token_out: String,
        recipient: Option<String>,
        minimum_receive: Uint128,
    ) -> Result<Response, ContractError> {
        let lp_token = AssetInfo::Native(lp_token_out.clone());
        let pool = P::get_pool_for_lp_token(deps.as_ref(), &lp_token)?;
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
                P::simulate_noswap_join(deps.as_ref(), &lp_token, &assets)?;

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
            for asset in assets.into_iter() {
                if asset.amount > Uint128::zero() {
                    let msg = CallbackMsg::SingleSidedJoin {
                        asset: asset.clone(),
                        lp_token: lp_token_out.clone(),
                    }
                    .into_cosmos_msg(&env)?;
                    provide_res = provide_res.add_message(msg);
                }
            }

            provide_res
        };

        // Query current contract LP token balance
        let lp_token_balance = lp_token.query_balance(&deps.querier, &env.contract.address)?;

        // Callback to return LP tokens
        let callback_msg = CallbackMsg::ReturnLpTokens {
            balance_before: Asset {
                info: lp_token,
                amount: lp_token_balance,
            },
            recipient,
            minimum_receive,
        }
        .into_cosmos_msg(&env)?;

        let event =
            Event::new("rover/zapper/execute_provide_liquidity").add_attributes(event_attrs);
        Ok(response.add_message(callback_msg).add_event(event))
    }

    fn execute_withdraw_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: Option<String>,
    ) -> Result<Response, ContractError> {
        // Make sure only one coin is sent
        one_coin(&info)?;
        let lp_token: Asset = info.funds[0].clone().into();

        let pool = P::get_pool_for_lp_token(deps.as_ref(), &lp_token.info)?;

        // Simulate withdraw liquidity
        let assets_returned = pool.simulate_withdraw_liquidity(deps.as_ref(), &lp_token)?;

        let withdraw_liquidity_res =
            pool.withdraw_liquidity(deps.as_ref(), &env, lp_token.clone())?;

        // Unwrap recipient or use caller
        let recipient = recipient.map_or(Ok(info.sender), |x| deps.api.addr_validate(&x))?;

        // Send LP tokens to recipient
        let send_msgs = assets_returned.transfer_msgs(&recipient)?;

        let event = Event::new("rover/zapper/execute_withdraw_liquidity")
            .add_attribute("lp_token", lp_token.info.to_string())
            .add_attribute("assets_returned", assets_returned.to_string())
            .add_attribute("recipient", recipient);

        Ok(withdraw_liquidity_res
            .add_messages(send_msgs)
            .add_event(event))
    }

    /// CallbackMsg handler to provide liquidity with the given assets. This needs
    /// to be a callback, rather than doing in the first ExecuteMsg, because
    /// pool.provide_liquidity does a simulation with current reserves, and our
    /// actions in the top level ExecuteMsg will alter the reserves, which means the
    /// reserves would be wrong in the provide liquidity simulation.
    pub fn execute_callback_single_sided_join(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        asset: Asset,
        lp_token: String,
    ) -> Result<Response, ContractError> {
        let assets = AssetList::from(vec![asset.clone()]);

        let lp_token = AssetInfo::Native(lp_token);
        let pool = P::get_pool_for_lp_token(deps.as_ref(), &lp_token)?;
        let res = pool.provide_liquidity(deps.as_ref(), &env, assets, Uint128::one())?;

        let event = Event::new("rover/zapper/execute_callback_single_sided_join")
            .add_attribute("asset", asset.to_string());

        Ok(res.add_event(event))
    }

    fn execute_return_tokens(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        balance_before: Asset,
        recipient: Addr,
        minimum_receive: Uint128,
    ) -> Result<Response, ContractError> {
        let balance_after = balance_before.query_balance(&deps.querier, &env.contract.address)?;
        let return_amount = balance_after.checked_sub(balance_before.amount)?;

        // Assert return_amount is greater than minimum_receive
        if return_amount < minimum_receive {
            return Err(ContractError::InsufficientLpTokens {
                expected: minimum_receive,
                received: return_amount,
            });
        }

        let asset = Asset {
            info: balance_before.info,
            amount: return_amount,
        };
        let send_msg = asset.transfer_msg(&recipient)?;

        let event = Event::new("rover/zapper/execute_callback_return_lp_tokens")
            .add_attribute("assets", asset.to_string())
            .add_attribute("recipient", recipient);

        Ok(Response::new().add_message(send_msg).add_event(event))
    }

    fn query_estimate_provide_liquidity(
        deps: Deps,
        env: Env,
        lp_token_out: String,
        coins_in: Vec<Coin>,
    ) -> StdResult<Binary> {
        let lp_token = AssetInfo::Native(lp_token_out);
        let pool = P::get_pool_for_lp_token(deps, &lp_token)?;

        let mut assets: AssetList = coins_in.into();

        let lp_tokens_returned = if assets.len() == 1 {
            let lp_tokens_returned = pool.simulate_provide_liquidity(deps, &env, assets)?;
            lp_tokens_returned.amount
        } else {
            let (mut lp_tokens_received, tokens_used) =
                P::simulate_noswap_join(deps, &lp_token, &assets)?;

            // Deduct tokens used to get remaining tokens
            assets.deduct_many(&tokens_used)?;

            for asset in assets.into_iter() {
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

    fn query_estimate_withdraw_liquidity(
        deps: Deps,
        _env: Env,
        coin_in: Coin,
    ) -> StdResult<Binary> {
        let lp_token: Asset = coin_in.into();
        let pool = P::get_pool_for_lp_token(deps, &lp_token.info)?;

        let assets_returned = pool.simulate_withdraw_liquidity(deps, &lp_token)?;

        let native_assets_returned: Vec<Coin> = assets_returned
            .into_iter()
            .filter_map(|x| match &x.info {
                AssetInfo::Native(token) => Some(Coin {
                    denom: token.clone(),
                    amount: x.amount,
                }),
                _ => None,
            })
            .collect();

        to_binary(&native_assets_returned)
    }
}
