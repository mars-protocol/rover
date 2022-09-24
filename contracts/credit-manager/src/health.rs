use cosmwasm_std::{Decimal, Deps, Env, Event, Response};
use mars_health::health::Health;

use rover::error::{ContractError, ContractResult};
use rover::traits::Coins;

use crate::query::query_positions;
use crate::state::{ORACLE, RED_BANK};
use crate::vault::simulate_withdraw;

pub fn compute_health(deps: Deps, env: &Env, account_id: &str) -> ContractResult<Health> {
    let res = query_positions(deps, env, account_id)?;
    let coins_if_withdrawn = simulate_withdraw(&deps, &res.vaults)?;

    let mut collateral = vec![];
    collateral.extend(res.coins);
    collateral.extend(coins_if_withdrawn);

    let oracle = ORACLE.load(deps.storage)?;
    let red_bank = RED_BANK.load(deps.storage)?;
    let health = Health::compute_health_from_coins(
        &deps.querier,
        oracle.address(),
        red_bank.address(),
        &collateral,
        &res.debts.to_coins(),
    )?;

    Ok(health)
}

pub fn assert_below_max_ltv(deps: Deps, env: Env, account_id: &str) -> ContractResult<Response> {
    let health = compute_health(deps, &env, account_id)?;

    if health.is_above_max_ltv() {
        return Err(ContractError::AboveMaxLTV {
            account_id: account_id.to_string(),
            max_ltv_health_factor: val_or_na(health.max_ltv_health_factor),
        });
    }

    let event = Event::new("position_changed")
        .add_attribute("timestamp", env.block.time.seconds().to_string())
        .add_attribute("height", env.block.height.to_string())
        .add_attribute("account_id", account_id)
        .add_attribute("assets_value", health.total_collateral_value.to_string())
        .add_attribute("debts_value", health.total_debt_value.to_string())
        .add_attribute(
            "lqdt_health_factor",
            val_or_na(health.liquidation_health_factor),
        )
        .add_attribute("liquidatable", health.is_liquidatable().to_string())
        .add_attribute(
            "max_ltv_health_factor",
            val_or_na(health.max_ltv_health_factor),
        )
        .add_attribute("above_max_ltv", health.is_above_max_ltv().to_string());

    Ok(Response::new()
        .add_attribute("action", "rover/credit_manager/callback/assert_health")
        .add_event(event))
}

pub fn val_or_na(opt: Option<Decimal>) -> String {
    opt.map_or_else(|| "n/a".to_string(), |dec| dec.to_string())
}
