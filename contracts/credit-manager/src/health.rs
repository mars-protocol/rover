use cosmwasm_std::{Decimal, Deps, Env, Event, Response};
use mars_health::health::Health;

use rover::error::{ContractError, ContractResult};
use rover::NftTokenId;

use crate::query::query_position;
use crate::state::{ORACLE, RED_BANK};
use crate::utils::debt_shares_to_amount;

pub fn compute_health(deps: Deps, env: &Env, token_id: NftTokenId) -> ContractResult<Health> {
    let res = query_position(deps, token_id)?;
    let debt_amounts = res
        .debt
        .iter()
        .map(|item| debt_shares_to_amount(deps, &env.contract.address, &item.denom, item.shares))
        .collect::<ContractResult<Vec<_>>>()?;

    let oracle = ORACLE.load(deps.storage)?;
    let red_bank = RED_BANK.load(deps.storage)?;
    let health = Health::compute_health_from_coins(
        &deps.querier,
        oracle.address(),
        red_bank.address(),
        &res.coins,
        debt_amounts.as_slice(),
    )?;

    Ok(health)
}

pub fn assert_below_max_ltv(
    deps: Deps,
    env: Env,
    token_id: NftTokenId,
) -> ContractResult<Response> {
    let health = compute_health(deps, &env, token_id)?;

    if health.is_above_max_ltv() {
        return Err(ContractError::AboveMaxLTV);
    }

    // if position.above_max_ltv {
    //     return Err(ContractError::AboveMaxLTV {
    //         token_id: token_id.to_string(),
    //         max_ltv_health_factor: val_or_not_applicable(position.max_ltv_health_factor),
    //     });
    // }

    let event = Event::new("position_changed")
        .add_attribute("timestamp", env.block.time.seconds().to_string())
        .add_attribute("height", env.block.height.to_string())
        .add_attribute("token_id", token_id)
        .add_attribute("assets_value", health.total_collateral_value.to_string())
        .add_attribute("debts_value", health.total_debt_value.to_string())
        .add_attribute(
            "lqdt_health_factor",
            val_or_not_applicable(health.liquidation_health_factor),
        )
        .add_attribute("liquidatable", health.is_liquidatable().to_string())
        .add_attribute(
            "max_ltv_health_factor",
            val_or_not_applicable(health.max_ltv_health_factor),
        )
        .add_attribute("above_max_ltv", health.is_above_max_ltv().to_string());

    Ok(Response::new()
        .add_attribute("action", "rover/credit_manager/callback/assert_health")
        .add_event(event))
}

fn val_or_not_applicable(opt: Option<Decimal>) -> String {
    opt.map_or_else(|| "n/a".to_string(), |dec| dec.to_string())
}
