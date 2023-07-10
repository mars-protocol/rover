<<<<<<< HEAD
use cosmwasm_std::{Addr, Coin, Deps, DepsMut, Env, Response, Storage, Uint128};
use mars_rover::{
    error::{ContractError, ContractResult},
    msg::execute::{ActionAmount, ActionCoin},
=======
use cosmwasm_std::{Coin, Deps, DepsMut, Env, Response, Uint128};
use mars_rover::{
    error::{ContractError, ContractResult},
    msg::execute::ActionCoin,
>>>>>>> b39a268d217f07def8721892266b0ddf0d38657b
};

use crate::{
    reclaim::lent_amount_to_shares,
<<<<<<< HEAD
    state::{LENT_SHARES, RED_BANK, TOTAL_LENT_SHARES},
    utils::{assert_coin_is_whitelisted, decrement_coin_balance, lent_shares_to_amount},
=======
    state::{COIN_BALANCES, LENT_SHARES, RED_BANK, TOTAL_LENT_SHARES},
    utils::{assert_coin_is_whitelisted, decrement_coin_balance},
>>>>>>> b39a268d217f07def8721892266b0ddf0d38657b
};

pub static DEFAULT_LENT_SHARES_PER_COIN: Uint128 = Uint128::new(1_000_000);

pub fn lend(
    mut deps: DepsMut,
    env: Env,
    account_id: &str,
    coin: &ActionCoin,
) -> ContractResult<Response> {
<<<<<<< HEAD
    is_zero_balance(deps.as_ref(), account_id, coin)?;
=======
>>>>>>> b39a268d217f07def8721892266b0ddf0d38657b
    assert_coin_is_whitelisted(&mut deps, &coin.denom)?;
    let amount_to_lend = &Coin {
        denom: coin.denom.to_string(),
        amount: coin.amount.value().unwrap_or(Uint128::MAX),
    };

<<<<<<< HEAD
    // total rover lent amount in Redbank for asset
=======
    let amount_to_lend = Coin {
        denom: coin.denom.to_string(),
        amount: get_lend_amount(deps.as_ref(), account_id, coin)?,
    };

    // Total Credit Manager has lent to Red Bank for denom
>>>>>>> b39a268d217f07def8721892266b0ddf0d38657b
    let red_bank = RED_BANK.load(deps.storage)?;
    let total_lent = red_bank.query_lent(&deps.querier, rover_addr, denom)?;

    let lent_shares_to_add = if total_lent.is_zero() {
        amount_to_lend.amount.checked_mul(DEFAULT_LENT_SHARES_PER_COIN)?
    } else {
<<<<<<< HEAD
        lent_amount_to_shares(deps.as_ref(), &env, amount_to_lend)?
=======
        lent_amount_to_shares(deps.as_ref(), &env, &amount_to_lend)?
>>>>>>> b39a268d217f07def8721892266b0ddf0d38657b
    };

    let add_shares = |shares: Option<Uint128>| -> ContractResult<Uint128> {
        Ok(shares.unwrap_or_else(Uint128::zero).checked_add(lent_shares_to_add)?)
    };

    TOTAL_LENT_SHARES.update(deps.storage, &amount_to_lend.denom, add_shares)?;
    LENT_SHARES.update(deps.storage, (account_id, &amount_to_lend.denom), add_shares)?;
<<<<<<< HEAD

    assert_lend_amount(deps.storage, account_id, amount_to_lend, total_lent)?;
    decrement_coin_balance(deps.storage, account_id, amount_to_lend)?;

    let red_bank = RED_BANK.load(deps.storage)?;
    let red_bank_lend_msg = red_bank.lend_msg(amount_to_lend)?;
=======

    decrement_coin_balance(deps.storage, account_id, &amount_to_lend)?;

    let red_bank_lend_msg = red_bank.lend_msg(&amount_to_lend)?;
>>>>>>> b39a268d217f07def8721892266b0ddf0d38657b

    Ok(Response::new()
        .add_message(red_bank_lend_msg)
        .add_attribute("action", "lend")
        .add_attribute("account_id", account_id)
        .add_attribute("lent_shares_added", lent_shares_to_add)
<<<<<<< HEAD
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
=======
        .add_attribute("coin_lent", amount_to_lend.denom))
>>>>>>> b39a268d217f07def8721892266b0ddf0d38657b
}

/// Queries balance to ensure passing EXACT is not too high.
/// Also asserts the amount is greater than zero.
fn get_lend_amount(deps: Deps, account_id: &str, coin: &ActionCoin) -> ContractResult<Uint128> {
    let amount_to_lend = if let Some(amount) = coin.amount.value() {
        amount
    } else {
        COIN_BALANCES.may_load(deps.storage, (account_id, &coin.denom))?.unwrap_or(Uint128::zero())
    };

    if amount_to_lend.is_zero() {
        Err(ContractError::NoAmount)
    } else {
        Ok(amount_to_lend)
    }
}

fn calculate_total_lent(deps: Deps, rover_addr: &Addr, denom: &str) -> ContractResult<Uint128> {
    // total rover lent amount in Redbank for asset
    let red_bank = RED_BANK.load(deps.storage)?;
    let total_lent_amount = red_bank.query_lent(&deps.querier, rover_addr, denom)?;

    Ok(total_lent_amount)
}
