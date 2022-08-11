use std::ops::{Add, Div, Sub};

use cosmwasm_std::{Coin, Decimal, DepsMut, Env, Response, StdError};

use rover::error::{ContractError, ContractResult};
use rover::health::Health;
use rover::msg::execute::CallbackMsg;
use rover::msg::query::CoinValue;
use rover::repay::RepayCalculation;

use crate::health::compute_health;
use crate::repay::repay_calculation;
use crate::state::{COIN_BALANCES, MAX_CLOSE_FACTOR, MAX_LIQUIDATION_BONUS, ORACLE};
use crate::utils::{coin_value, decrement_coin_balance, increment_coin_balance, IntoUint128};

pub fn liquidate_coin(
    deps: DepsMut,
    env: Env,
    liquidator_token_id: &str,
    liquidatee_token_id: &str,
    debt_coin: Coin,
    request_coin: Coin,
) -> ContractResult<Response> {
    // Assert the liquidatee's credit account is liquidatable
    let health = compute_health(deps.as_ref(), &env, liquidatee_token_id)?;
    if !health.liquidatable {
        return Err(ContractError::NotLiquidatable {
            token_id: liquidatee_token_id.to_string(),
            lqdt_health_factor: health
                .lqdt_health_factor
                .map_or("n/a".to_string(), |hf| hf.to_string()),
        });
    }

    let (debt, request) = adjust_liquidation_request(
        &deps,
        &env,
        liquidatee_token_id,
        &debt_coin,
        &request_coin,
        &health,
    )?;

    // Transfer debt coin from liquidator's coin balance to liquidatee
    // Will be used to pay off the debt via CallbackMsg::Repay {}
    decrement_coin_balance(deps.storage, liquidator_token_id, &debt.clone().into())?;
    increment_coin_balance(deps.storage, liquidatee_token_id, &debt.clone().into())?;
    let repay_msg = (CallbackMsg::Repay {
        token_id: liquidatee_token_id.to_string(),
        coin: debt.clone().into(),
    })
    .into_cosmos_msg(&env.contract.address)?;

    // Increment the requested coin amount in liquidator's account assets
    increment_coin_balance(deps.storage, liquidator_token_id, &request.clone().into())?;

    // Decrement the requested coin amount in liquidatee's coin balance
    decrement_coin_balance(deps.storage, liquidatee_token_id, &request.clone().into())?;

    // Ensure health factor has improved as a consequence of liquidation event
    let new_health = compute_health(deps.as_ref(), &env, liquidatee_token_id)?;
    if let Some(new_hf) = new_health.lqdt_health_factor {
        let prev_hf = health.lqdt_health_factor.unwrap(); // safe unwrap given it was liquidatable
        if prev_hf > new_hf {
            return Err(ContractError::HealthNotImproved {
                prev_hf: prev_hf.to_string(),
                new_hf: new_hf.to_string(),
            });
        }
    }

    Ok(Response::new()
        .add_message(repay_msg)
        .add_attribute("action", "rover/credit_manager/liquidate")
        .add_attribute("liquidatee_token_id", liquidatee_token_id)
        .add_attribute("debt_repaid_denom", debt.denom)
        .add_attribute("debt_repaid_amount", debt.amount)
        .add_attribute("request_coin_denom", request.denom)
        .add_attribute("request_coin_amount", request.amount)
        .add_attribute(
            "close_factor",
            debt.value.div(health.total_debts_value).to_string(),
        ))
}

/// In the case the liquidator does not send the right input values, we want to proactively fix them
/// as to ensure that the liquidation has a high chance of succeeding. We are adjusting in the case
/// that the debt coin amount and/or the request coin amounts are too high.
fn adjust_liquidation_request(
    deps: &DepsMut,
    env: &Env,
    liquidatee_token_id: &str,
    debt_coin: &Coin,
    request_coin: &Coin,
    health: &Health,
) -> ContractResult<(CoinValue, CoinValue)> {
    // Ensure debt to pay does not exceed max debt available
    let RepayCalculation {
        amount_to_repay: total_debt_amount,
        ..
    } = repay_calculation(deps, env, liquidatee_token_id, debt_coin)?;

    // Ensure that the debt they are paying is not more than close factor % of the total debt they have.
    let close_factor = MAX_CLOSE_FACTOR.load(deps.storage)?;
    let max_close_value = close_factor.checked_mul(health.total_debts_value)?;
    let oracle = ORACLE.load(deps.storage)?;
    let price = oracle.query_price(&deps.querier, &debt_coin.denom)?;
    let max_close_amount = max_close_value.div(price).uint128();

    // Get final debt to pay
    let max_debt_amount = *vec![debt_coin.amount, total_debt_amount, max_close_amount]
        .iter()
        .min()
        .ok_or_else(|| StdError::generic_err("Minimum not found"))?;
    let debt = coin_value(
        &deps.as_ref(),
        &Coin {
            denom: debt_coin.denom.clone(),
            amount: max_debt_amount,
        },
    )?;

    // Ensure coin request amount does not exceed the max the liquidatee has
    let liquidatee_balance = COIN_BALANCES
        .load(
            deps.storage,
            (liquidatee_token_id, request_coin.denom.as_str()),
        )
        .map_err(|_| ContractError::CoinNotAvailable(request_coin.denom.clone()))?;

    // Ensure the request_coin value is within range for adjusted debt
    // FORMULA: request amount = ((1 + liquidation bonus %) * debt value) / request asset price
    let liq_bonus_rate = MAX_LIQUIDATION_BONUS.load(deps.storage)?;
    let price = oracle.query_price(&deps.querier, &request_coin.denom)?;
    let debt_adjusted_max = liq_bonus_rate
        .add(Decimal::one())
        .checked_mul(debt.value)?
        .div(price)
        .uint128();

    let max_request_amount = *vec![request_coin.amount, liquidatee_balance, debt_adjusted_max]
        .iter()
        .min()
        .ok_or_else(|| StdError::generic_err("Minimum not found"))?;

    let request = coin_value(
        &deps.as_ref(),
        &Coin {
            denom: request_coin.denom.clone(),
            amount: max_request_amount,
        },
    )?;

    // Assert diffs do not exceed liquidation bonus %. If liquidator has accurate calculations, this
    // will ensure they at worst will lose their anticipated liquidation bonus amount. Beyond this
    // may result in too high of liquidation "slippage" for the liquidator.
    let request_diff = Decimal::from_ratio(request.amount, request_coin.amount);
    if Decimal::one().sub(request_diff) > liq_bonus_rate {
        return Err(ContractError::VarianceTooHigh {
            denom: request_coin.denom.clone(),
            sent: request_coin.amount,
            adjusted: request.amount,
            diff: request_diff.to_string(),
            max_diff_allowed: liq_bonus_rate.to_string(),
        });
    }
    Ok((debt, request))
}
