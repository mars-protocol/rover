use cosmwasm_std::{
    to_binary, Coin, CosmosMsg, DepsMut, Env, Reply, Response, SubMsg, Uint128, WasmMsg,
};

use rover::adapters::{Vault, VaultBase, VaultPosition};
use rover::error::{ContractError, ContractResult};
use rover::extensions::AttrParse;
use rover::msg::vault::{ExecuteMsg, UnlockingTokens};
use rover::NftTokenId;

use crate::state::{VAULT_POSITIONS, VAULT_REQUEST_TEMP_AMOUNT_VAR, VAULT_REQUEST_TEMP_TOKEN_VAR};

pub const VAULT_REQUEST_REPLY_ID: u64 = 2;

pub fn request_unlock_from_vault(
    deps: DepsMut,
    token_id: NftTokenId,
    vault: Vault,
    shares: Uint128,
) -> ContractResult<Response> {
    let vault_info = vault.query_vault_info(&deps.querier)?;
    if vault_info.lockup.is_none() {
        return Err(ContractError::RequirementsNotMet(
            "This vault does not require lockup. Call withdraw directly.".to_string(),
        ));
    }

    VAULT_REQUEST_TEMP_TOKEN_VAR.save(deps.storage, &token_id.to_string())?;
    VAULT_REQUEST_TEMP_AMOUNT_VAR.save(deps.storage, &shares)?;

    let unlock_msg = SubMsg::reply_on_success(
        CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: vault.0.to_string(),
            funds: vec![Coin {
                denom: vault_info.token_denom,
                amount: shares,
            }],
            msg: to_binary(&ExecuteMsg::RequestUnlock {})?,
        }),
        VAULT_REQUEST_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(unlock_msg)
        .add_attribute("action", "rover/credit_manager/vault/request_unlock"))
}

pub fn handle_unlock_request_reply(
    deps: DepsMut,
    env: Env,
    reply: Reply,
) -> ContractResult<Response> {
    let token_id = VAULT_REQUEST_TEMP_TOKEN_VAR.load(deps.storage)?;
    let shares = VAULT_REQUEST_TEMP_AMOUNT_VAR.load(deps.storage)?;

    let unlock_event = reply.parse_unlock_event()?;
    let vault_addr = deps.api.addr_validate(unlock_event.vault_addr.as_str())?;
    let vault = VaultBase(vault_addr.clone());
    let vault_info = vault.query_vault_info(&deps.querier)?;

    VAULT_POSITIONS.update(
        deps.storage,
        (token_id.as_str(), vault_addr),
        |position_opt| {
            if position_opt.is_none() || position_opt.clone().unwrap().locked < shares {
                return Err(ContractError::NotEnoughFunds {});
            }

            let mut p = position_opt.unwrap();
            p.unlocking.push(UnlockingTokens {
                id: unlock_event.id,
                amount: shares,
                unlocked_at: env.block.time.plus_seconds(vault_info.lockup.unwrap()),
            });

            Ok(VaultPosition {
                unlocked: p.unlocked,
                locked: p.locked - shares,
                unlocking: p.unlocking,
            })
        },
    )?;

    Ok(Response::new().add_attribute("action", "rover/credit_manager/vault/withdraw/handle_reply"))
}
