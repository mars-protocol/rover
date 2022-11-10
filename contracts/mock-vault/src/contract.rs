#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coin, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, Uint128};
use cosmwasm_vault_standard::extensions::force_unlock::ForceUnlockExecuteMsg;
use cosmwasm_vault_standard::extensions::lockup::{LockupExecuteMsg, LockupQueryMsg};
use cosmwasm_vault_standard::msg::{
    ExtensionExecuteMsg, ExtensionQueryMsg, VaultStandardExecuteMsg, VaultStandardQueryMsg,
};

use crate::deposit::deposit;
use crate::error::ContractResult;
use crate::msg::InstantiateMsg;
use crate::query::{
    query_lockup_duration, query_unlocking_position, query_unlocking_positions, query_vault_info,
    query_vault_token_supply, shares_to_base_denom_amount,
};
use crate::state::{
    CHAIN_BANK, COIN_BALANCE, LOCKUP_TIME, NEXT_LOCKUP_ID, ORACLE, TOTAL_VAULT_SHARES,
    VAULT_TOKEN_DENOM,
};
use crate::unlock::{request_unlock, withdraw_unlocked, withdraw_unlocking_force};
use crate::withdraw::{redeem_force, withdraw};

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
    COIN_BALANCE.save(deps.storage, &coin(0, msg.base_token_denom))?;
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
    msg: VaultStandardExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        VaultStandardExecuteMsg::Deposit { .. } => deposit(deps, info),
        VaultStandardExecuteMsg::Redeem { .. } => withdraw(deps, info),
        VaultStandardExecuteMsg::VaultExtension(ext) => match ext {
            ExtensionExecuteMsg::Lockup(lockup_msg) => match lockup_msg {
                LockupExecuteMsg::WithdrawUnlocked { lockup_id, .. } => {
                    withdraw_unlocked(deps, env, &info.sender, lockup_id)
                }
                LockupExecuteMsg::Unlock { .. } => request_unlock(deps, env, info),
            },
            ExtensionExecuteMsg::ForceUnlock(force_msg) => match force_msg {
                ForceUnlockExecuteMsg::ForceRedeem { .. } => redeem_force(deps, info),
                ForceUnlockExecuteMsg::ForceWithdrawUnlocking {
                    lockup_id, amount, ..
                } => withdraw_unlocking_force(deps, &info.sender, lockup_id, amount),
                _ => unimplemented!(),
            },
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: VaultStandardQueryMsg) -> ContractResult<Binary> {
    let res = match msg {
        VaultStandardQueryMsg::TotalVaultTokenSupply {} => {
            to_binary(&query_vault_token_supply(deps.storage)?)
        }
        VaultStandardQueryMsg::Info {} => to_binary(&query_vault_info(deps)?),
        VaultStandardQueryMsg::PreviewRedeem { amount } => {
            to_binary(&shares_to_base_denom_amount(deps.storage, amount)?)
        }
        VaultStandardQueryMsg::VaultExtension(ext) => match ext {
            ExtensionQueryMsg::Lockup(lockup_msg) => match lockup_msg {
                LockupQueryMsg::UnlockingPositions { owner, .. } => {
                    to_binary(&query_unlocking_positions(deps, owner)?)
                }
                LockupQueryMsg::UnlockingPosition { lockup_id, .. } => {
                    to_binary(&query_unlocking_position(deps, lockup_id)?)
                }
                LockupQueryMsg::LockupDuration {} => to_binary(&query_lockup_duration(deps)?),
            },
        },
        _ => unimplemented!(),
    };
    res.map_err(Into::into)
}
