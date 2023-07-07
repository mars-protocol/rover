use cosmwasm_std::{Addr, Coin, Deps, DepsMut, Env, Response, Storage, Uint128};
use mars_rover::{
    error::{ContractError, ContractResult},
    msg::execute::{ActionAmount, ActionCoin},
};

use crate::{
    reclaim::lent_amount_to_shares,
    state::{LENT_SHARES, RED_BANK, TOTAL_LENT_SHARES},
    utils::{assert_coin_is_whitelisted, decrement_coin_balance, lent_shares_to_amount},
};

pub static DEFAULT_LENT_SHARES_PER_COIN: Uint128 = Uint128::new(1_000_000);

pub fn lend(
    mut deps: DepsMut,
    env: Env,
    account_id: &str,
    coin: &ActionCoin,
) -> ContractResult<Response> {
    is_zero_balance(deps.as_ref(), account_id, &coin)?;
    assert_coin_is_whitelisted(&mut deps, &coin.denom)?;

    let total_lent =
        calculate_total_lent(deps.as_ref(), &env.contract.address, &coin.denom.to_string())?;

    let amount_to_lend = &Coin {
        denom: coin.denom.to_string(),
        amount: coin.amount.value().unwrap_or(Uint128::MAX),
    };

    let lent_shares_to_add = if total_lent.is_zero() {
        default_lent_amount(deps.as_ref(), &env, &amount_to_lend)?
    } else {
        lent_amount_to_shares(deps.as_ref(), &env, &amount_to_lend)?
    };

    let add_shares = |shares: Option<Uint128>| -> ContractResult<Uint128> {
        Ok(shares.unwrap_or_else(Uint128::zero).checked_add(lent_shares_to_add)?)
    };

    TOTAL_LENT_SHARES.update(deps.storage, &amount_to_lend.denom, add_shares)?;
    LENT_SHARES.update(deps.storage, (account_id, &amount_to_lend.denom), add_shares)?;

    assert_lend_amount(deps.storage, account_id, &amount_to_lend, total_lent)?;
    decrement_coin_balance(deps.storage, account_id, &amount_to_lend)?;

    let red_bank = RED_BANK.load(deps.storage)?;
    let red_bank_lend_msg = red_bank.lend_msg(&amount_to_lend)?;

    Ok(Response::new()
        .add_message(red_bank_lend_msg)
        .add_attribute("action", "lend")
        .add_attribute("account_id", account_id)
        .add_attribute("lent_shares_added", lent_shares_to_add)
        .add_attribute("coin_lent", &amount_to_lend.denom))
}

fn is_zero_balance(deps: Deps, account_id: &str, coin: &ActionCoin) -> ContractResult<()> {
    match coin.amount {
        ActionAmount::Exact(a) => {
            if a.is_zero() {
                return Err(ContractError::NoAmount);
            }
        }
        ActionAmount::AccountBalance => {
            let balance = LENT_SHARES
                .may_load(deps.storage, (account_id, &coin.denom))?
                .unwrap_or(Uint128::zero());
            if balance.is_zero() {
                return Err(ContractError::NoAmount);
            }
        }
    }
    Ok(())
}

pub fn current_lent_for_denom(
    deps: Deps,
    env: &Env,
    account_id: &str,
    denom: &str,
) -> ContractResult<Uint128> {
    let lent_shares =
        LENT_SHARES.load(deps.storage, (account_id, denom)).map_err(|_| ContractError::NoneLent)?;
    let coin = lent_shares_to_amount(deps, &env.contract.address, denom, lent_shares)?;
    Ok(coin.amount)
}

/// A guard to ensure once a user makes a lend, the amount they can reclaim is >= 1.
/// Due to integer rounding, if the pool shares issued are quite large and the lend action
/// amount is low, it could round down to zero.
fn assert_lend_amount(
    storage: &dyn Storage,
    account_id: &str,
    coin_to_lend: &Coin,
    total_lent: Uint128,
) -> ContractResult<()> {
    let total_shares = TOTAL_LENT_SHARES.load(storage, &coin_to_lend.denom)?;
    let user_shares = LENT_SHARES.load(storage, (account_id, &coin_to_lend.denom))?;

    if total_lent
        .checked_add(coin_to_lend.amount)?
        .checked_mul_floor((user_shares, total_shares))?
        .is_zero()
    {
        return Err(ContractError::NoAmount);
    }
    Ok(())
}

fn default_lent_amount(_deps: Deps, _env: &Env, coin: &Coin) -> ContractResult<Uint128> {
    let amount = coin.amount.checked_mul(DEFAULT_LENT_SHARES_PER_COIN)?;
    Ok(amount)
}

fn calculate_total_lent(deps: Deps, rover_addr: &Addr, denom: &str) -> ContractResult<Uint128> {
    // total rover lent amount in Redbank for asset
    let red_bank = RED_BANK.load(deps.storage)?;
    let total_lent_amount = red_bank.query_lent(&deps.querier, rover_addr, denom)?;

    Ok(total_lent_amount)
}
