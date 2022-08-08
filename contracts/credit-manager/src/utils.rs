use cosmwasm_std::{Addr, Coin, DepsMut, QuerierWrapper, StdError, StdResult, Storage, Uint128};
use cw721::OwnerOfResponse;
use cw721_base::QueryMsg;

use rover::adapters::Vault;
use rover::error::{ContractError, ContractResult};
use rover::extensions::Stringify;

use crate::state::{ACCOUNT_NFT, ALLOWED_COINS, ALLOWED_VAULTS, COIN_BALANCES};

pub fn assert_vault_is_whitelisted(storage: &mut dyn Storage, vault: &Vault) -> ContractResult<()> {
    let is_whitelisted = ALLOWED_VAULTS.has(storage, vault.address());
    if !is_whitelisted {
        return Err(ContractError::NotWhitelisted(vault.address().to_string()));
    }
    Ok(())
}

pub fn assert_denoms_match_vault_reqs(
    querier: QuerierWrapper,
    vault: &Vault,
    assets: &[Coin],
) -> ContractResult<()> {
    let vault_info = vault.query_vault_info(&querier)?;

    let all_req_coins_present = vault_info
        .assets
        .iter()
        .all(|coin| assets.iter().any(|req_coin| req_coin.denom == coin.denom));

    if !all_req_coins_present || assets.len() != vault_info.assets.len() {
        return Err(ContractError::RequirementsNotMet(format!(
            "Required assets: {} do not match given assets: {}",
            vault_info.assets.as_slice().to_string(),
            assets.to_string()
        )));
    }
    Ok(())
}

pub fn assert_coins_are_whitelisted(
    storage: &mut dyn Storage,
    assets: &[Coin],
) -> ContractResult<()> {
    assets
        .iter()
        .try_for_each(|asset| assert_coin_is_whitelisted(storage, asset))
}

pub fn assert_coin_is_whitelisted(storage: &mut dyn Storage, coin: &Coin) -> ContractResult<()> {
    let is_whitelisted = ALLOWED_COINS.has(storage, &coin.denom);
    if !is_whitelisted {
        return Err(ContractError::NotWhitelisted(coin.denom.clone()));
    }
    Ok(())
}

pub fn assert_is_token_owner(deps: &DepsMut, user: &Addr, token_id: &str) -> ContractResult<()> {
    let contract_addr = ACCOUNT_NFT.load(deps.storage)?;
    let owner_res: OwnerOfResponse = deps.querier.query_wasm_smart(
        contract_addr,
        &QueryMsg::OwnerOf {
            token_id: token_id.to_string(),
            include_expired: None,
        },
    )?;

    if user != &owner_res.owner {
        return Err(ContractError::NotTokenOwner {
            user: user.to_string(),
            token_id: token_id.to_string(),
        });
    }

    Ok(())
}

pub fn increment_position(storage: &mut dyn Storage, token_id: &str, coin: &Coin) -> StdResult<()> {
    COIN_BALANCES.update(
        storage,
        (token_id, &coin.denom),
        |value_opt| -> StdResult<_> {
            value_opt
                .unwrap_or_else(Uint128::zero)
                .checked_add(coin.amount)
                .map_err(StdError::overflow)
        },
    )?;
    Ok(())
}
