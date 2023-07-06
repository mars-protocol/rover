use cosmwasm_std::{Coin, Deps, DepsMut, Env, Response, Uint128};
use mars_rover::msg::execute::ActionAmount;
use mars_rover::{
    error::{ContractError, ContractResult},
    msg::execute::ActionCoin,
};
use std::cmp::min;

use crate::reclaim::lent_amount_to_shares;
use crate::utils::lent_shares_to_amount;
use crate::{
    state::{LENT_SHARES, RED_BANK, TOTAL_LENT_SHARES},
    utils::{assert_coin_is_whitelisted, decrement_coin_balance},
};

pub static DEFAULT_LENT_SHARES_PER_COIN: Uint128 = Uint128::new(1_000_000);

pub fn lend(
    mut deps: DepsMut,
    env: Env,
    account_id: &str,
    coin: ActionCoin,
) -> ContractResult<Response> {
    // get lent shares for this account
    // convert lent shares to lent amount
    // min(lent_amount, coin.amount.value().unwrap_or(Uint128::MAX))  understand what this does, follow logic of repay
    is_zero_balance(deps.as_ref(), account_id, &coin)?;
    assert_coin_is_whitelisted(&mut deps, &coin.denom)?;

    let lent_amount = current_lent_for_denom(deps.as_ref(), &env, account_id, &coin.denom)?;
    let amount_to_lend = min(lent_amount, coin.amount.value().unwrap_or(Uint128::MAX));

    let coin_to_lend = Coin {
        denom: coin.denom.to_string(),
        amount: amount_to_lend,
    };

    let lent_shares_to_add = lent_amount_to_shares(deps.as_ref(), &env, &coin_to_lend)?;

    let add_shares = |shares: Option<Uint128>| -> ContractResult<Uint128> {
        Ok(shares.unwrap_or_else(Uint128::zero).checked_add(lent_shares_to_add)?)
    };

    TOTAL_LENT_SHARES.update(deps.storage, &coin_to_lend.denom, add_shares)?;
    LENT_SHARES.update(deps.storage, (account_id, &coin_to_lend.denom), add_shares)?;

    decrement_coin_balance(deps.storage, account_id, &coin_to_lend)?;

    let red_bank = RED_BANK.load(deps.storage)?;
    let red_bank_lend_msg = red_bank.lend_msg(&coin_to_lend)?;

    Ok(Response::new()
        .add_message(red_bank_lend_msg)
        .add_attribute("action", "lend")
        .add_attribute("account_id", account_id)
        .add_attribute("lent_shares_added", lent_shares_to_add)
        .add_attribute("coin_lent", format!("{}{}", amount_to_lend, &coin.denom)))
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
