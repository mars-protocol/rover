use std::marker::PhantomData;

use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Uint128,
};
use cw_asset::{Asset, AssetInfo};
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
            } => P::execute_provide_liquidity(
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
                        P::execute_callback_single_sided_join(deps, env, info, asset, lp_token)
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
            } => P::query_estimate_provide_liquidity(deps, env, lp_token_out, coins_in),
            QueryMsg::EstimateWithdrawLiquidity { coin_in } => {
                Self::query_estimate_withdraw_liquidity(deps, env, coin_in)
            }
        }
    }

    fn execute_withdraw_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        recipient: Option<String>,
    ) -> Result<Response, ContractError> {
        // Make sure only one coin is sent
        one_coin(&info)?;

        let lp_token_coin = info.funds[0].clone();
        let pool = P::get_pool_for_lp_token(deps.as_ref(), &lp_token_coin.denom)?;
        let lp_token: Asset = lp_token_coin.into();

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

    fn query_estimate_withdraw_liquidity(
        deps: Deps,
        _env: Env,
        coin_in: Coin,
    ) -> StdResult<Binary> {
        let pool = P::get_pool_for_lp_token(deps, &coin_in.denom)?;
        let lp_token: Asset = coin_in.into();

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
