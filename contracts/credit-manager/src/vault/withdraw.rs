use cosmwasm_std::{
    to_binary, Coin, CosmosMsg, DepsMut, Env, Reply, Response, StdResult, SubMsg, Uint128, WasmMsg,
};

use rover::adapters::{Vault, VaultPosition};
use rover::error::{ContractError, ContractResult};
use rover::extensions::{AssetTransferMsg, AttrParse};
use rover::msg::vault::ExecuteMsg;
use rover::{NftTokenId, Shares};

use crate::state::{TOTAL_VAULT_SHARES, VAULT_POSITIONS, VAULT_WITHDRAW_TEMP_VAR};
use crate::utils::{assert_vault_is_whitelisted, increment_position};

pub const VAULT_WITHDRAW_REPLY_ID: u64 = 1;

pub fn withdraw_from_vault(
    deps: DepsMut,
    token_id: NftTokenId,
    vault: Vault,
    shares: Uint128,
    force: bool,
) -> ContractResult<Response> {
    assert_vault_is_whitelisted(deps.storage, &vault)?;

    VAULT_POSITIONS.update(
        deps.storage,
        (token_id, vault.0.clone()),
        |p_opt| match p_opt {
            None => Err(ContractError::NotEnoughFunds {}),
            Some(p) => Ok(VaultPosition {
                unlocked: p.unlocked - shares,
                locked: p.locked,
                unlocking: p.unlocking,
            }),
        },
    )?;

    TOTAL_VAULT_SHARES.update(
        deps.storage,
        vault.0.clone(),
        |total_shares_opt| -> ContractResult<Shares> {
            let total_shares = total_shares_opt.ok_or_else(|| ContractError::NotEnoughShares {
                needed: shares,
                actual: Uint128::zero(),
            })?;
            Ok(total_shares - shares)
        },
    )?;

    // Sends LP shares to vault in exchange for assets. Updating token_id's asset positions happens
    // in reply message after those assets are sent back. Temporary variable used to track token_id.
    VAULT_WITHDRAW_TEMP_VAR.save(deps.storage, &token_id.to_string())?;
    let vault_info = vault.query_vault_info(&deps.querier)?;
    let withdraw_msg = SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: vault.0.to_string(),
            funds: vec![Coin {
                denom: vault_info.token_denom,
                amount: shares,
            }],
            msg: to_binary(
                &(if force {
                    ExecuteMsg::ForceWithdraw {}
                } else {
                    ExecuteMsg::Withdraw {}
                }),
            )?,
        }),
        VAULT_WITHDRAW_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(withdraw_msg)
        .add_attribute("action", "rover/credit_manager/vault/withdraw"))
}

pub fn handle_withdraw_reply(deps: DepsMut, env: Env, reply: Reply) -> ContractResult<Response> {
    let asset_transfer_msg = reply.parse_transfer_msg()?;
    assert_assets_sent_to_rover(env, &asset_transfer_msg)?;

    let token_id = VAULT_WITHDRAW_TEMP_VAR.load(deps.storage)?;
    increment_asset_positions(deps, token_id, asset_transfer_msg)?;

    Ok(Response::new().add_attribute("action", "rover/credit_manager/vault/withdraw/handle_reply"))
}

pub fn increment_asset_positions(
    deps: DepsMut,
    token_id: String,
    asset_transfer_msg: AssetTransferMsg,
) -> StdResult<()> {
    asset_transfer_msg
        .amount
        .iter()
        .try_for_each(|coin| increment_position(deps.storage, token_id.as_str(), coin))
}

pub fn assert_assets_sent_to_rover(
    env: Env,
    asset_transfer_msg: &AssetTransferMsg,
) -> Result<(), ContractError> {
    if asset_transfer_msg.recipient != env.contract.address {
        Err(ContractError::RequirementsNotMet(
            "Assets were not sent back to Rover contract".to_string(),
        ))
    } else {
        Ok(())
    }
}
