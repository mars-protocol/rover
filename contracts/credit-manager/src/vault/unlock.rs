use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, Reply, Response, SubMsg, Uint128, WasmMsg};

use rover::adapters::{Vault, VaultPosition};
use rover::error::{ContractError, ContractResult};
use rover::extensions::AttrParse;
use rover::msg::vault::ExecuteMsg;
use rover::{NftTokenId, Shares};

use crate::state::{TOTAL_VAULT_SHARES, VAULT_POSITIONS, VAULT_UNLOCK_TEMP_VAR};
use crate::utils::assert_vault_is_whitelisted;
use crate::vault::{assert_assets_sent_to_rover, increment_asset_positions};

pub const VAULT_WITHDRAW_UNLOCKED_REPLY_ID: u64 = 3;

pub fn withdraw_unlocked_from_vault(
    deps: DepsMut,
    env: Env,
    token_id: NftTokenId,
    vault: Vault,
    position_id: Uint128,
) -> ContractResult<Response> {
    assert_vault_is_whitelisted(deps.storage, &vault)?;

    let vault_position = VAULT_POSITIONS
        .may_load(deps.storage, (token_id, vault.address().clone()))?
        .ok_or_else(|| ContractError::NoPosition(token_id.to_string()))?;

    let matching_unlock = vault_position
        .unlocking
        .iter()
        .find(|p| p.id == position_id)
        .ok_or_else(|| ContractError::NoPositionMatch(position_id.to_string()))?;

    let matching_unlock = vault.query_unlocking_position_info(&deps.querier, matching_unlock.id)?;

    if matching_unlock.unlocked_at > env.block.time {
        return Err(ContractError::UnlockNotReady {});
    }

    VAULT_POSITIONS.save(
        deps.storage,
        (token_id, vault.address().clone()),
        &VaultPosition {
            unlocked: vault_position.unlocked,
            locked: vault_position.locked,
            unlocking: vault_position
                .unlocking
                .iter()
                .filter(|p| p.id != position_id)
                .map(Clone::clone)
                .collect(),
        },
    )?;

    TOTAL_VAULT_SHARES.update(
        deps.storage,
        vault.address().clone(),
        |total_shares_opt| -> ContractResult<Shares> {
            let total_shares = total_shares_opt.ok_or_else(|| ContractError::NotEnoughShares {
                needed: matching_unlock.amount,
                actual: Uint128::zero(),
            })?;
            Ok(total_shares - matching_unlock.amount)
        },
    )?;

    VAULT_UNLOCK_TEMP_VAR.save(deps.storage, &token_id.to_string())?;

    let unlock_msg = SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: vault.address().to_string(),
            funds: vec![],
            msg: to_binary(&ExecuteMsg::WithdrawUnlocked { id: position_id })?,
        }),
        VAULT_WITHDRAW_UNLOCKED_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(unlock_msg)
        .add_attribute("action", "rover/credit_manager/vault/unlock"))
}

pub fn handle_withdraw_unlocked_reply(
    deps: DepsMut,
    env: Env,
    reply: Reply,
) -> ContractResult<Response> {
    let asset_transfer_msg = reply.parse_transfer_msg()?;
    assert_assets_sent_to_rover(env, &asset_transfer_msg)?;

    let token_id = VAULT_UNLOCK_TEMP_VAR.load(deps.storage)?;
    increment_asset_positions(deps, token_id, asset_transfer_msg)?;

    Ok(Response::new().add_attribute(
        "action",
        "rover/credit_manager/vault/withdraw_unlocked/handle_reply",
    ))
}
