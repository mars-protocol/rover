use std::ops::{Add, Sub};

use cosmwasm_std::Decimal;

use mars_rover::msg::query::HealthResponse;

pub const DEFAULT_MAX_VALUE_FOR_BURN: u128 = 1000u128;

pub fn above_max_for_burn() -> HealthResponse {
    HealthResponse {
        total_debt_value: Decimal::from_atomics(DEFAULT_MAX_VALUE_FOR_BURN.add(1), 0).unwrap(),
        total_collateral_value: Default::default(),
        max_ltv_adjusted_collateral: Default::default(),
        liquidation_threshold_adjusted_collateral: Default::default(),
        max_ltv_health_factor: None,
        liquidation_health_factor: None,
        liquidatable: false,
        above_max_ltv: false,
    }
}

pub fn at_max_for_burn() -> HealthResponse {
    HealthResponse {
        total_debt_value: Decimal::from_atomics(DEFAULT_MAX_VALUE_FOR_BURN, 0).unwrap(),
        total_collateral_value: Default::default(),
        max_ltv_adjusted_collateral: Default::default(),
        liquidation_threshold_adjusted_collateral: Default::default(),
        max_ltv_health_factor: None,
        liquidation_health_factor: None,
        liquidatable: false,
        above_max_ltv: false,
    }
}

pub fn below_max_for_burn() -> HealthResponse {
    HealthResponse {
        total_debt_value: Decimal::from_atomics(DEFAULT_MAX_VALUE_FOR_BURN.sub(1), 0).unwrap(),
        total_collateral_value: Default::default(),
        max_ltv_adjusted_collateral: Default::default(),
        liquidation_threshold_adjusted_collateral: Default::default(),
        max_ltv_health_factor: None,
        liquidation_health_factor: None,
        liquidatable: false,
        above_max_ltv: false,
    }
}
