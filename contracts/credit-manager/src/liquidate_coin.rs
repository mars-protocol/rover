use std::ops::Add;

use cosmwasm_std::{
    Coin, CosmosMsg, Decimal, Deps, DepsMut, Env, QuerierWrapper, Response, StdError, Storage,
    Uint128,
};
use mars_rover::{
    adapters::oracle::Oracle,
    error::{ContractError, ContractResult},
    msg::execute::{ActionAmount, ActionCoin, CallbackMsg},
    traits::Stringify,
};

use crate::{
    health::query_health,
    reclaim::prepare_reclaim_state_and_msg,
    repay::current_debt_for_denom,
    state::{COIN_BALANCES, LENT_SHARES, MAX_CLOSE_FACTOR, ORACLE, RED_BANK},
    utils::{decrement_coin_balance, increment_coin_balance, lent_shares_to_amount},
};

pub fn liquidate_coin(
    mut deps: DepsMut,
    env: Env,
    liquidator_account_id: &str,
    liquidatee_account_id: &str,
    debt_coin: Coin,
    request_coin_denom: &str,
) -> ContractResult<Response> {
    let request_coin_balance = COIN_BALANCES
        .load(deps.storage, (liquidatee_account_id, request_coin_denom))
        .map_err(|_| ContractError::CoinNotAvailable(request_coin_denom.to_string()))?;

    // Check how much lent coin is available for reclaim (can be withdrawn from Red Bank)
    let (total_lent_amount, _) =
        current_lent(deps.as_ref(), &env, liquidatee_account_id, request_coin_denom)?;

    // Check total amount of requested coin in liquidatee account and lent to Red Bank
    let request_coin_total_amt = request_coin_balance.checked_add(total_lent_amount)?;

    let (debt, request) = calculate_liquidation(
        &deps,
        &env,
        liquidatee_account_id,
        &debt_coin,
        request_coin_denom,
        request_coin_total_amt,
    )?;

    let mut response = Response::new();

    let repay_msg =
        repay_debt(deps.storage, &env, liquidator_account_id, liquidatee_account_id, &debt)?;
    response = response.add_message(repay_msg);

    // Check if we have to reclaim remaining amount
    if request.amount > request_coin_balance {
        let reclaim_amount = request.amount - request_coin_balance;
        // Using Reclaim callback and:
        // - subtracting reclaimed amount (which is later incremented in Reclaim action)
        //   from current liquidatee coin balance could result with balance below 0 (not possible).
        // - adding Transfer callback (transferring requested coin from liquidatee to liquidator)
        //   could leave liquidator in unhealthy state (liquidated coin will be transferred
        //   to the liquidator after CallbackMsg::AssertMaxLTV check).
        let (msg, _, _, _) = prepare_reclaim_state_and_msg(
            deps.branch(),
            env,
            liquidatee_account_id,
            &ActionCoin {
                denom: request_coin_denom.to_string(),
                amount: ActionAmount::Exact(reclaim_amount),
            },
        )?;
        response = response.add_message(msg);
    }

    // Transfer requested coin from liquidatee to liquidator
    decrement_coin_balance(deps.storage, liquidatee_account_id, &request)?;
    increment_coin_balance(deps.storage, liquidator_account_id, &request)?;

    Ok(response
        .add_attribute("action", "liquidate_coin")
        .add_attribute("account_id", liquidator_account_id)
        .add_attribute("liquidatee_account_id", liquidatee_account_id)
        .add_attribute("coin_debt_repaid", debt.to_string())
        .add_attribute("coin_liquidated", request.to_string()))
}

/// Calculates precise debt & request coin amounts to liquidate
/// The debt amount will be adjusted down if:
/// - Exceeds liquidatee's total debt for denom
/// - Not enough liquidatee request coin balance to match
/// - The value of the debt repaid exceeds the maximum close factor %
/// Returns -> (Debt Coin, Request Coin)
pub fn calculate_liquidation(
    deps: &DepsMut,
    env: &Env,
    liquidatee_account_id: &str,
    debt_coin: &Coin,
    request_coin: &str,
    request_coin_balance: Uint128,
) -> ContractResult<(Coin, Coin)> {
    // Assert the liquidatee's credit account is liquidatable
    let health = query_health(deps.as_ref(), liquidatee_account_id)?;
    if !health.liquidatable {
        return Err(ContractError::NotLiquidatable {
            account_id: liquidatee_account_id.to_string(),
            lqdt_health_factor: health.liquidation_health_factor.to_string(),
        });
    }

    // Ensure debt repaid does not exceed liquidatee's total debt for denom
    let (total_debt_amount, _) =
        current_debt_for_denom(deps.as_ref(), env, liquidatee_account_id, &debt_coin.denom)?;

    // Ensure debt amount does not exceed close factor % of the liquidatee's total debt value
    let close_factor = MAX_CLOSE_FACTOR.load(deps.storage)?;
    let max_close_value = health.total_debt_value.checked_mul_floor(close_factor)?;
    let oracle = ORACLE.load(deps.storage)?;
    let debt_res = oracle.query_price(&deps.querier, &debt_coin.denom)?;
    let max_close_amount = max_close_value.checked_div_floor(debt_res.price)?;

    // Calculate the maximum debt possible to repay given liquidatee's request coin balance
    // FORMULA: debt amount = request value / (1 + liquidation bonus %) / debt price
    let request_res = oracle.query_price(&deps.querier, request_coin)?;
    let max_request_value = request_coin_balance.checked_mul_floor(request_res.price)?;
    let liq_bonus_rate = RED_BANK
        .load(deps.storage)?
        .query_market(&deps.querier, &debt_coin.denom)?
        .liquidation_bonus;
    let request_coin_adjusted_max_debt = max_request_value
        .checked_div_floor(Decimal::one().add(liq_bonus_rate))?
        .checked_div_floor(debt_res.price)?;

    let final_debt_to_repay = *vec![
        debt_coin.amount,
        total_debt_amount,
        max_close_amount,
        request_coin_adjusted_max_debt,
    ]
    .iter()
    .min()
    .ok_or_else(|| StdError::generic_err("Minimum not found"))?;

    // Calculate exact request coin amount to give to liquidator
    // FORMULA: request amount = debt value * (1 + liquidation bonus %) / request coin price
    let request_amount = final_debt_to_repay
        .checked_mul_floor(debt_res.price)?
        .checked_mul_floor(liq_bonus_rate.add(Decimal::one()))?
        .checked_div_floor(request_res.price)?;

    // (Debt Coin, Request Coin)
    let result = (
        Coin {
            denom: debt_coin.denom.clone(),
            amount: final_debt_to_repay,
        },
        Coin {
            denom: request_coin.to_string(),
            amount: request_amount,
        },
    );

    assert_liquidation_profitable(&deps.querier, &oracle, result.clone())?;

    Ok(result)
}

pub fn repay_debt(
    storage: &mut dyn Storage,
    env: &Env,
    liquidator_account_id: &str,
    liquidatee_account_id: &str,
    debt: &Coin,
) -> ContractResult<CosmosMsg> {
    // Transfer debt coin from liquidator's coin balance to liquidatee
    // Will be used to pay off the debt via CallbackMsg::Repay {}
    decrement_coin_balance(storage, liquidator_account_id, debt)?;
    increment_coin_balance(storage, liquidatee_account_id, debt)?;
    let msg = (CallbackMsg::Repay {
        account_id: liquidatee_account_id.to_string(),
        coin: debt.into(),
    })
    .into_cosmos_msg(&env.contract.address)?;
    Ok(msg)
}

/// In scenarios with small amounts or large gap between coin prices, there is a possibility
/// that the liquidation will result in loss for the liquidator. This assertion prevents this.
fn assert_liquidation_profitable(
    querier: &QuerierWrapper,
    oracle: &Oracle,
    (debt_coin, request_coin): (Coin, Coin),
) -> ContractResult<()> {
    let debt_value = oracle.query_total_value(querier, &[debt_coin.clone()])?;
    let request_value = oracle.query_total_value(querier, &[request_coin.clone()])?;

    if debt_value >= request_value {
        return Err(ContractError::LiquidationNotProfitable {
            debt_coin,
            request_coin,
        });
    }

    Ok(())
}

fn current_lent(
    deps: Deps,
    env: &Env,
    account_id: &str,
    denom: &str,
) -> ContractResult<(Uint128, Uint128)> {
    let lent_shares_opt = LENT_SHARES.may_load(deps.storage, (account_id, denom))?;
    match lent_shares_opt {
        Some(lent_shares) => {
            let coin = lent_shares_to_amount(deps, &env.contract.address, denom, lent_shares)?;
            Ok((coin.amount, lent_shares))
        }
        None => Ok((Uint128::zero(), Uint128::zero())),
    }
}
