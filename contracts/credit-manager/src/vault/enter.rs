use cosmwasm_std::{
    coin as c, to_binary, Addr, Coin, CosmosMsg, Deps, DepsMut, QuerierWrapper, Response, Uint128,
    WasmMsg,
};

use rover::adapters::vault::{UpdateType, Vault, VaultPositionUpdate};
use rover::error::{ContractError, ContractResult};
use rover::msg::execute::CallbackMsg;
use rover::msg::ExecuteMsg;

use crate::state::{ORACLE, VAULT_CONFIGS};
use crate::utils::{assert_coins_are_whitelisted, decrement_coin_balance};
use crate::vault::utils::{assert_vault_is_whitelisted, update_vault_position};

pub fn enter_vault(
    deps: DepsMut,
    rover_addr: &Addr,
    account_id: &str,
    vault: Vault,
    coin: Coin,
) -> ContractResult<Response> {
    assert_coins_are_whitelisted(deps.storage, vec![coin.denom.as_str()])?;
    assert_vault_is_whitelisted(deps.storage, &vault)?;
    assert_denom_matches_vault_reqs(deps.querier, &vault, &coin)?;
    assert_deposit_is_under_cap(deps.as_ref(), &vault, &coin, rover_addr)?;

    decrement_coin_balance(deps.storage, account_id, &coin)?;

    let current_balance = vault.query_balance(&deps.querier, rover_addr)?;
    let update_vault_balance_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: rover_addr.to_string(),
        funds: vec![],
        msg: to_binary(&ExecuteMsg::Callback(CallbackMsg::UpdateVaultCoinBalance {
            vault: vault.clone(),
            account_id: account_id.to_string(),
            previous_total_balance: current_balance,
        }))?,
    });

    Ok(Response::new()
        .add_message(vault.deposit_msg(&coin)?)
        .add_message(update_vault_balance_msg)
        .add_attribute("action", "rover/credit_manager/vault/deposit"))
}

pub fn update_vault_coin_balance(
    deps: DepsMut,
    vault: Vault,
    account_id: &str,
    previous_total_balance: Uint128,
    rover_addr: &Addr,
) -> ContractResult<Response> {
    let current_balance = vault.query_balance(&deps.querier, rover_addr)?;

    if previous_total_balance >= current_balance {
        return Err(ContractError::NoVaultCoinsReceived);
    }

    let diff = current_balance.checked_sub(previous_total_balance)?;
    let duration = vault.query_lockup_duration(&deps.querier).ok();

    update_vault_position(
        deps.storage,
        account_id,
        &vault.address,
        match duration {
            None => VaultPositionUpdate::Unlocked(UpdateType::Increment(diff)),
            Some(_) => VaultPositionUpdate::Locked(UpdateType::Increment(diff)),
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "rover/credit_manager/vault/update_balance")
        .add_attribute(
            "amount_incremented",
            current_balance.checked_sub(previous_total_balance)?,
        ))
}

pub fn assert_denom_matches_vault_reqs(
    querier: QuerierWrapper,
    vault: &Vault,
    coin: &Coin,
) -> ContractResult<()> {
    let vault_info = vault.query_info(&querier)?;
    if vault_info.req_denom != coin.denom {
        return Err(ContractError::RequirementsNotMet(format!(
            "Required coin: {} -- does not match given coin: {}",
            vault_info.req_denom, coin.denom
        )));
    }
    Ok(())
}

pub fn assert_deposit_is_under_cap(
    deps: Deps,
    vault: &Vault,
    coin: &Coin,
    rover_addr: &Addr,
) -> ContractResult<()> {
    let oracle = ORACLE.load(deps.storage)?;
    let deposit_request_value = oracle.query_total_value(&deps.querier, &[coin.clone()])?;

    let config = VAULT_CONFIGS.load(deps.storage, &vault.address)?;
    let deposit_cap_value = oracle.query_total_value(&deps.querier, &[config.deposit_cap])?;

    let vault_info = vault.query_info(&deps.querier)?;
    let rover_vault_coin_balance = vault.query_balance(&deps.querier, rover_addr)?;
    let rover_vault_coins_value = oracle.query_total_value(
        &deps.querier,
        &[c(
            rover_vault_coin_balance.u128(),
            vault_info.vault_token_denom,
        )],
    )?;

    let new_total_vault_value = rover_vault_coins_value.checked_add(deposit_request_value)?;

    if new_total_vault_value > deposit_cap_value {
        return Err(ContractError::AboveVaultDepositCap {
            new_value: new_total_vault_value.to_string(),
            maximum: deposit_cap_value.to_string(),
        });
    }

    Ok(())
}