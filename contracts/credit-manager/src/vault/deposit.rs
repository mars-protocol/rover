use cosmwasm_std::{
    Coin, DepsMut, Env, QuerierWrapper, Reply, Response, StdError, StdResult, Storage, Uint128,
};

use rover::adapters::{Vault, VaultPosition};
use rover::error::{ContractError, ContractResult};
use rover::extensions::{AttrParse, CoinTransferMsg, Stringify};
use rover::NftTokenId;

use crate::state::{
    ALLOWED_COINS, TEMP_VAULT_DEPOSIT_TOKEN_ID, TEMP_VAULT_DEPOSIT_VAULT, TOTAL_VAULT_COIN_BALANCE,
    VAULT_POSITIONS,
};
use crate::utils::{assert_vault_is_whitelisted, decrement_coin_balance};

pub fn deposit_into_vault(
    deps: DepsMut,
    token_id: NftTokenId,
    vault: Vault,
    coins: &[Coin],
) -> ContractResult<Response> {
    assert_coins_are_whitelisted(deps.storage, coins)?;
    assert_vault_is_whitelisted(deps.storage, &vault)?;
    assert_denoms_match_vault_reqs(deps.querier, &vault, coins)?;

    // Decrement token's coin balance amount
    coins.iter().try_for_each(|coin| -> ContractResult<_> {
        decrement_coin_balance(deps.storage, token_id, coin)?;
        Ok(())
    })?;

    // To later access on submessage reply
    TEMP_VAULT_DEPOSIT_TOKEN_ID.save(deps.storage, &token_id.to_string())?;
    TEMP_VAULT_DEPOSIT_VAULT.save(deps.storage, &vault)?;

    Ok(Response::new()
        .add_submessage(vault.deposit_msg(coins)?)
        .add_attribute("action", "rover/credit_manager/vault/deposit"))
}

pub fn handle_vault_deposit_reply(
    deps: DepsMut,
    env: Env,
    reply: Reply,
) -> ContractResult<Response> {
    let transfer_msg = reply.parse_transfer_msg()?;
    let vault_coin = assert_vault_transfer(env, &transfer_msg)?;

    let token_id = TEMP_VAULT_DEPOSIT_TOKEN_ID.load(deps.storage)?;
    let vault = TEMP_VAULT_DEPOSIT_VAULT.load(deps.storage)?;

    // increment token's position
    let vault_info = vault.query_vault_info(&deps.querier)?;
    VAULT_POSITIONS.update(
        deps.storage,
        (&token_id, vault.address().clone()),
        |position_opt| -> ContractResult<_> {
            let p = position_opt.unwrap_or(VaultPosition {
                unlocked: Uint128::zero(),
                locked: Uint128::zero(),
            });
            match vault_info.lockup {
                None => Ok(VaultPosition {
                    unlocked: p.unlocked + vault_coin.amount,
                    locked: p.locked,
                }),
                Some(_) => Ok(VaultPosition {
                    unlocked: p.unlocked,
                    locked: p.locked + vault_coin.amount,
                }),
            }
        },
    )?;

    // Increment total vault coins
    TOTAL_VAULT_COIN_BALANCE.update(
        deps.storage,
        vault.address().clone(),
        |shares_opt| -> ContractResult<_> {
            Ok(match shares_opt {
                None => vault_coin.amount,
                Some(total) => total + vault_coin.amount,
            })
        },
    )?;

    // purge temp vars
    TEMP_VAULT_DEPOSIT_TOKEN_ID.remove(deps.storage);
    TEMP_VAULT_DEPOSIT_VAULT.remove(deps.storage);

    Ok(Response::new().add_attribute("action", "rover/credit_manager/vault/deposit/handle_reply"))
}

pub fn assert_vault_transfer(env: Env, coin_transfer_msg: &CoinTransferMsg) -> StdResult<Coin> {
    if coin_transfer_msg.recipient != env.contract.address {
        Err(StdError::generic_err(
            "Vault coins were not sent back to Rover contract",
        ))
    } else if coin_transfer_msg.coins.len() != 1 {
        Err(StdError::generic_err("More than one token type sent back"))
    } else {
        let coin = coin_transfer_msg.coins.first().unwrap();
        Ok(coin.clone())
    }
}

pub fn assert_denoms_match_vault_reqs(
    querier: QuerierWrapper,
    vault: &Vault,
    assets: &[Coin],
) -> ContractResult<()> {
    let vault_info = vault.query_vault_info(&querier)?;

    let all_req_coins_present = vault_info
        .coins
        .iter()
        .all(|coin| assets.iter().any(|req_coin| req_coin.denom == coin.denom));

    if !all_req_coins_present || assets.len() != vault_info.coins.len() {
        return Err(ContractError::RequirementsNotMet(format!(
            "Required assets: {} -- do not match given assets: {}",
            vault_info.coins.as_slice().to_string(),
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
