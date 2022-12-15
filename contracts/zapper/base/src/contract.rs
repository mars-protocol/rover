use std::marker::PhantomData;

use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Uint128,
};
use cw_asset::{Asset, AssetInfo, AssetList};

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
                    CallbackMsg::ReturnTokens {
                        balance_before,
                        recipient,
                    } => Self::execute_return_tokens(deps, env, info, balance_before, recipient),
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
        let assets: AssetList = info.funds.clone().into();

        // Unwrap recipient or use caller
        let recipient =
            recipient.map_or_else(|| Ok(info.sender.clone()), |x| deps.api.addr_validate(&x))?;

        // Query contracts balance before providing liquidity
        let balance_before = lp_token.query_balance(&deps.querier, &env.contract.address)?;

        let provide_liquidity_res =
            pool.provide_liquidity(deps.as_ref(), &env, assets, minimum_receive)?;

        // Send LP tokens to recipient
        let callback_msg = CallbackMsg::ReturnTokens {
            balance_before: Asset {
                info: lp_token,
                amount: balance_before,
            },
            recipient,
        }
        .into_cosmos_msg(&env)?;

        let event = Event::new("rover/zapper/execute_provide_liquidity")
            .add_attribute("lp_token_out", lp_token_out)
            .add_attribute("minimum_receive", minimum_receive);

        Ok(provide_liquidity_res
            .add_message(callback_msg)
            .add_event(event))
    }

    fn execute_withdraw_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: Option<String>,
    ) -> Result<Response, ContractError> {
        // Make sure only one coin is sent
        if info.funds.len() != 1 {
            return Err("More than one coin sent to Withdraw Liquidity".into());
        }
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

    fn execute_return_tokens(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        balance_before: Asset,
        recipient: Addr,
    ) -> Result<Response, ContractError> {
        let balance_after = balance_before.query_balance(&deps.querier, &env.contract.address)?;
        let amount = balance_after.checked_sub(balance_before.amount)?;
        let asset = Asset {
            info: balance_before.info,
            amount,
        };
        let send_msg = asset.transfer_msg(&recipient)?;

        let event = Event::new("rover/zapper/execute_return_tokens")
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

        let lp_tokens_returned = pool.simulate_provide_liquidity(deps, &env, coins_in.into())?;

        to_binary(&lp_tokens_returned.amount)
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
