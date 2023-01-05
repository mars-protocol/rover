use std::cmp::min;

use cosmwasm_std::{Coin, Deps, DepsMut, Env, Response, Uint256};

use mars_coin::Coin256;
use mars_rover::error::{ContractError, ContractResult};
use mars_rover::msg::execute::ActionCoin;

use crate::state::{DEBT_SHARES, RED_BANK, TOTAL_DEBT_SHARES};
use crate::utils::{debt_shares_to_amount, decrement_coin_balance};

pub fn repay(
    deps: DepsMut,
    env: Env,
    account_id: &str,
    coin: &ActionCoin,
) -> ContractResult<Response> {
    // Ensure repayment does not exceed max debt on account
    let (debt_amount, debt_shares) =
        current_debt_for_denom(deps.as_ref(), &env, account_id, &coin.denom)?;
    let amount_to_repay = min(debt_amount, coin.amount.value().unwrap_or(Uint256::MAX));
    let shares_to_repay = debt_amount_to_shares(
        deps.as_ref(),
        &env,
        &Coin256 {
            denom: coin.denom.to_string(),
            amount: amount_to_repay,
        },
    )?;

    // Decrement token's debt position
    if amount_to_repay == debt_amount {
        DEBT_SHARES.remove(deps.storage, (account_id, &coin.denom));
    } else {
        DEBT_SHARES.save(
            deps.storage,
            (account_id, &coin.denom),
            &debt_shares.checked_sub(shares_to_repay)?,
        )?;
    }

    // Decrement total debt shares for coin
    let total_debt_shares = TOTAL_DEBT_SHARES.load(deps.storage, &coin.denom)?;
    TOTAL_DEBT_SHARES.save(
        deps.storage,
        &coin.denom,
        &total_debt_shares.checked_sub(shares_to_repay)?,
    )?;

    decrement_coin_balance(
        deps.storage,
        account_id,
        &Coin256 {
            denom: coin.denom.to_string(),
            amount: amount_to_repay,
        },
    )?;

    let red_bank = RED_BANK.load(deps.storage)?;
    let red_bank_repay_msg = red_bank.repay_msg(&Coin {
        denom: coin.denom.to_string(),
        amount: amount_to_repay.try_into()?,
    })?;

    Ok(Response::new()
        .add_message(red_bank_repay_msg)
        .add_attribute("action", "rover/credit-manager/repay")
        .add_attribute("debt_shares_repaid", shares_to_repay)
        .add_attribute("coins_repaid", amount_to_repay))
}

fn debt_amount_to_shares(deps: Deps, env: &Env, coin: &Coin256) -> ContractResult<Uint256> {
    let red_bank = RED_BANK.load(deps.storage)?;
    let total_debt_shares = TOTAL_DEBT_SHARES.load(deps.storage, &coin.denom)?;
    let total_debt_amount =
        red_bank.query_debt(&deps.querier, &env.contract.address, &coin.denom)?;
    let shares = total_debt_shares.checked_multiply_ratio(coin.amount, total_debt_amount)?;
    Ok(shares)
}

/// Get token's current total debt for denom
/// Returns -> (debt amount, debt shares)
pub fn current_debt_for_denom(
    deps: Deps,
    env: &Env,
    account_id: &str,
    denom: &str,
) -> ContractResult<(Uint256, Uint256)> {
    let debt_shares = DEBT_SHARES
        .load(deps.storage, (account_id, denom))
        .map_err(|_| ContractError::NoDebt)?;
    let coin = debt_shares_to_amount(deps, &env.contract.address, denom, debt_shares)?;
    Ok((coin.amount, debt_shares))
}
