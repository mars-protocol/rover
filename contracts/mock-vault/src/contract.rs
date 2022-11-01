#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coin, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, Uint128};

use cosmos_vault_standard::extensions::lockup::{LockupExecuteMsg, LockupQueryMsg};
use cosmos_vault_standard::msg::{ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, QueryMsg};

use crate::deposit::deposit;
use crate::error::ContractResult;
use crate::msg::InstantiateMsg;
use crate::query::{
    query_lockup, query_lockup_duration, query_lockups, query_underlying_for_shares,
    query_vault_info, query_vault_token_supply,
};
use crate::state::{
    CHAIN_BANK, COIN_BALANCE, LOCKUP_TIME, NEXT_LOCKUP_ID, ORACLE, TOTAL_VAULT_SHARES,
    VAULT_TOKEN_DENOM,
};
use crate::unlock::{request_unlock, withdraw_unlocked, withdraw_unlocking_force};
use crate::withdraw::{withdraw, withdraw_force};

pub const STARTING_VAULT_SHARES: Uint128 = Uint128::new(1_000_000);

/// cw-multi-test does not yet have the ability to mint sdk coins. For this reason,
/// this contract expects to be pre-funded with vault tokens and it will simulate the mint.
pub const DEFAULT_VAULT_TOKEN_PREFUND: Uint128 = Uint128::new(1_000_000_000);

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    COIN_BALANCE.save(deps.storage, &coin(0, msg.req_denom))?;
    LOCKUP_TIME.save(deps.storage, &msg.lockup)?;
    ORACLE.save(deps.storage, &msg.oracle.check(deps.api)?)?;
    VAULT_TOKEN_DENOM.save(deps.storage, &msg.vault_token_denom)?;
    CHAIN_BANK.save(deps.storage, &DEFAULT_VAULT_TOKEN_PREFUND)?;
    NEXT_LOCKUP_ID.save(deps.storage, &1)?;
    TOTAL_VAULT_SHARES.save(deps.storage, &Uint128::zero())?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::Deposit { .. } => deposit(deps, info),
        ExecuteMsg::Redeem { .. } => withdraw(deps, info),
        ExecuteMsg::VaultExtension(ext) => match ext {
            ExtensionExecuteMsg::Lockup(lockup_msg) => match lockup_msg {
                LockupExecuteMsg::WithdrawUnlocked { lockup_id, .. } => {
                    withdraw_unlocked(deps, env, &info.sender, lockup_id)
                }
                LockupExecuteMsg::Unlock { .. } => request_unlock(deps, env, info),
                LockupExecuteMsg::ForceWithdraw { .. } => withdraw_force(deps, info),
                LockupExecuteMsg::ForceWithdrawUnlocking {
                    lockup_id, amount, ..
                } => withdraw_unlocking_force(deps, &info.sender, lockup_id, amount),
            },
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    let res = match msg {
        QueryMsg::TotalVaultTokenSupply {} => to_binary(&query_vault_token_supply(deps.storage)?),
        QueryMsg::Info {} => to_binary(&query_vault_info(deps)?),
        QueryMsg::PreviewRedeem { amount } => {
            to_binary(&query_underlying_for_shares(deps.storage, amount)?)
        }
        QueryMsg::VaultExtension(ext) => match ext {
            ExtensionQueryMsg::Lockup(lockup_msg) => match lockup_msg {
                LockupQueryMsg::Lockups { owner, .. } => to_binary(&query_lockups(deps, owner)?),
                LockupQueryMsg::Lockup { lockup_id, .. } => {
                    to_binary(&query_lockup(deps, lockup_id)?)
                }
                LockupQueryMsg::LockupDuration {} => to_binary(&query_lockup_duration(deps)?),
            },
        },
        _ => unimplemented!(),
    };
    res.map_err(Into::into)
}
