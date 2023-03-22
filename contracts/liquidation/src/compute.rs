use std::{collections::HashMap, ops::Add};

use cosmwasm_std::{
    Coin, Decimal, Deps, DepsMut, Env, QuerierWrapper, Response, StdError, StdResult, Uint128,
};
use mars_rover::{
    adapters::{
        oracle::Oracle,
        vault::{Vault, VaultPositionType},
    },
    error::{ContractError, ContractResult},
    msg::query::DebtAmount,
    traits::Stringify,
};
use mars_rover_health_types::HealthError::CreditManagerNotSet;

use crate::{
    querier::LiquidationQuerier, state::CREDIT_MANAGER, types::CreditManagerConfigResponse,
};

pub fn liquidate_deposit(
    deps: DepsMut,
    env: Env,
    liquidator_account_id: &str,
    liquidatee_account_id: &str,
    debt_coin: Coin,
    request_coin_denom: &str,
) -> ContractResult<Response> {
    unimplemented!()
}

pub fn liquidate_lend(
    deps: DepsMut,
    env: Env,
    liquidator_account_id: &str,
    liquidatee_account_id: &str,
    debt_coin: Coin,
    request_coin_denom: &str,
) -> ContractResult<Response> {
    unimplemented!()
}

pub fn liquidate_vault(
    deps: DepsMut,
    env: Env,
    liquidator_account_id: &str,
    liquidatee_account_id: &str,
    debt_coin: Coin,
    request_vault: Vault,
    position_type: VaultPositionType,
) -> ContractResult<Response> {
    unimplemented!()
}

pub fn compute_liquidation(
    deps: DepsMut,
    env: Env,
    liquidator_account_id: &str,
    liquidatee_account_id: &str,
    debt_coin: Coin,
    request_coin_denom: &str,
) -> ContractResult<(Coin, Coin)> {
    let credit_manager_addr =
        CREDIT_MANAGER.may_load(deps.storage)?.ok_or(CreditManagerNotSet {})?;
    let querier = LiquidationQuerier::new(&deps.querier, &credit_manager_addr);
    let config = querier.query_credit_manager_config()?;
    let positions = querier.query_positions(liquidatee_account_id)?;
    let debt = positions
        .debts
        .iter()
        .find(|d| d.denom.eq(&debt_coin.denom))
        .ok_or(ContractError::NoDebt)?;
    let lend = positions
        .lends
        .iter()
        .find(|d| d.denom.eq(request_coin_denom))
        .ok_or(ContractError::NoneLent)?;
    let deposit = positions
        .deposits
        .iter()
        .find(|d| d.denom.eq(request_coin_denom))
        .ok_or(ContractError::CoinNotAvailable(request_coin_denom.to_string()))?;

    let request_coin_balance = deposit.amount;
    calculate_liquidation(
        &deps,
        &env,
        liquidatee_account_id,
        &debt_coin,
        request_coin_denom,
        request_coin_balance,
        config,
        debt,
    )
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
    config: CreditManagerConfigResponse,
    liquidatee_debt_coin: &DebtAmount,
) -> ContractResult<(Coin, Coin)> {
    // Assert the liquidatee's credit account is liquidatable
    let health = config.health_contract.query_health(&deps.querier, liquidatee_account_id)?;
    if !health.liquidatable {
        return Err(ContractError::NotLiquidatable {
            account_id: liquidatee_account_id.to_string(),
            lqdt_health_factor: health.liquidation_health_factor.to_string(),
        });
    }

    // Ensure debt repaid does not exceed liquidatee's total debt for denom
    let total_debt_amount = liquidatee_debt_coin.amount;

    // Ensure debt amount does not exceed close factor % of the liquidatee's total debt value
    let max_close_value = health.total_debt_value.checked_mul_floor(config.max_close_factor)?;
    let debt_res = config.oracle.query_price(&deps.querier, &debt_coin.denom)?;
    let max_close_amount = max_close_value.checked_div_floor(debt_res.price)?;

    // Calculate the maximum debt possible to repay given liquidatee's request coin balance
    // FORMULA: debt amount = request value / (1 + liquidation bonus %) / debt price
    let request_res = config.oracle.query_price(&deps.querier, request_coin)?;
    let max_request_value = request_coin_balance.checked_mul_floor(request_res.price)?;
    let liq_bonus_rate =
        config.red_bank.query_market(&deps.querier, &debt_coin.denom)?.liquidation_bonus;
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

    assert_liquidation_profitable(&deps.querier, &config.oracle, result.clone())?;

    Ok(result)
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
