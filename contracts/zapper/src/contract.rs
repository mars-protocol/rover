#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, Event, MessageInfo, Response, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use cw_asset::{Asset, AssetInfo, AssetList};
use cw_dex::traits::Pool as PoolTrait;
use cw_dex::Pool;

use crate::error::ContractError;
use crate::msg::{CallbackMsg, ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(
        deps.storage,
        &format!("crates.io:{}", CONTRACT_NAME),
        CONTRACT_VERSION,
    )?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
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
        } => execute_provide_liquidity(deps, env, info, lp_token_out, recipient, minimum_receive),
        ExecuteMsg::WithdrawLiquidity { recipient } => {
            execute_withdraw_liquidity(deps, env, info, recipient)
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
                } => execute_return_tokens(deps, env, info, balance_before, recipient),
            }
        }
    }
}

pub fn execute_provide_liquidity(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    lp_token_out: String,
    recipient: Option<String>,
    minimum_receive: Uint128,
) -> Result<Response, ContractError> {
    let lp_token = AssetInfo::Native(lp_token_out.clone());
    let pool = Pool::get_pool_for_lp_token(deps.as_ref(), &lp_token)?;
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

pub fn execute_withdraw_liquidity(
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

    let pool = Pool::get_pool_for_lp_token(deps.as_ref(), &lp_token.info)?;

    // Simulate withdraw liquidity
    let assets_returned = pool.simulate_withdraw_liquidity(deps.as_ref(), &lp_token)?;

    let withdraw_liquidity_res = pool.withdraw_liquidity(deps.as_ref(), &env, lp_token.clone())?;

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

pub fn execute_return_tokens(
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::EstimateProvideLiquidity {
            lp_token_out,
            coins_in,
        } => query_estimate_provide_liquidity(deps, env, lp_token_out, coins_in),
        QueryMsg::EstimateWithdrawLiquidity { coin_in } => {
            query_estimate_withdraw_liquidity(deps, env, coin_in)
        }
    }
}

pub fn query_estimate_provide_liquidity(
    deps: Deps,
    env: Env,
    lp_token_out: String,
    coins_in: Vec<Coin>,
) -> StdResult<Binary> {
    let lp_token = AssetInfo::Native(lp_token_out);
    let pool = Pool::get_pool_for_lp_token(deps, &lp_token)?;

    let lp_tokens_returned = pool.simulate_provide_liquidity(deps, &env, coins_in.into())?;

    to_binary(&lp_tokens_returned.amount)
}

pub fn query_estimate_withdraw_liquidity(
    deps: Deps,
    _env: Env,
    coin_in: Coin,
) -> StdResult<Binary> {
    let lp_token: Asset = coin_in.into();
    let pool = Pool::get_pool_for_lp_token(deps, &lp_token.info)?;

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
