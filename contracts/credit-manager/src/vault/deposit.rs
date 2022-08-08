use cosmwasm_std::{Coin, DepsMut, Env, Response, Uint128};

use rover::adapters::{Vault, VaultPosition};
use rover::error::{ContractError, ContractResult};
use rover::NftTokenId;

use crate::state::{COIN_BALANCES, ORACLE, TOTAL_VAULT_SHARES, VAULT_POSITIONS};
use crate::utils::{
    assert_coins_are_whitelisted, assert_denoms_match_vault_reqs, assert_vault_is_whitelisted,
};

pub static DEFAULT_VAULT_SHARES: Uint128 = Uint128::new(1_000_000);

pub fn deposit_into_vault(
    deps: DepsMut,
    env: Env,
    token_id: NftTokenId,
    vault: Vault,
    coins: &[Coin],
) -> ContractResult<Response> {
    assert_coins_are_whitelisted(deps.storage, coins)?;
    assert_vault_is_whitelisted(deps.storage, &vault)?;
    assert_denoms_match_vault_reqs(deps.querier, &vault, coins)?;

    // Decrement token's coin balance amount
    coins.iter().try_for_each(|coin| -> ContractResult<_> {
        COIN_BALANCES.update(
            deps.storage,
            (token_id, &coin.denom),
            |amount_opt| -> ContractResult<_> {
                match amount_opt {
                    Some(current_amount) if current_amount > coin.amount => {
                        Ok(current_amount - coin.amount)
                    }
                    _ => Err(ContractError::NotEnoughFunds {}),
                }
            },
        )?;
        Ok(())
    })?;

    // Incrementing token's vault share position
    let oracle = ORACLE.load(deps.storage)?;
    let value_of_assets = oracle.query_total_value(&deps.querier, coins)?;
    let total_vault_value =
        vault.query_total_value(&deps.querier, &oracle, &env.contract.address)?;
    let total_shares = TOTAL_VAULT_SHARES
        .may_load(deps.storage, vault.address().clone())?
        .unwrap_or(Uint128::zero());
    let shares_to_add = if total_shares.is_zero() {
        DEFAULT_VAULT_SHARES
    } else {
        total_shares
            .checked_multiply_ratio(value_of_assets.atomics(), total_vault_value.atomics())?
    };

    let vault_info = vault.query_vault_info(&deps.querier)?;

    VAULT_POSITIONS.update(
        deps.storage,
        (token_id, vault.address().clone()),
        |position_opt| -> ContractResult<_> {
            let p = position_opt.unwrap_or(VaultPosition {
                unlocked: Uint128::zero(),
                locked: Uint128::zero(),
                unlocking: vec![],
            });
            match vault_info.lockup {
                None => Ok(VaultPosition {
                    unlocked: p.unlocked + shares_to_add,
                    locked: p.locked,
                    unlocking: p.unlocking,
                }),
                Some(_) => Ok(VaultPosition {
                    unlocked: p.unlocked,
                    locked: p.locked + shares_to_add,
                    unlocking: p.unlocking,
                }),
            }
        },
    )?;

    // Increment total shares
    TOTAL_VAULT_SHARES.update(
        deps.storage,
        vault.address().clone(),
        |shares_opt| -> ContractResult<_> {
            Ok(match shares_opt {
                None => shares_to_add,
                Some(total_shares) => total_shares + shares_to_add,
            })
        },
    )?;

    Ok(Response::new()
        .add_message(vault.deposit_msg(coins)?)
        .add_attribute("action", "rover/credit_manager/vault/deposit"))
}
