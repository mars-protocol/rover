use cosmwasm_std::{Addr, Decimal};
use mars_rover::{
    adapters::{health::HealthContract, oracle::Oracle, red_bank::RedBank},
    msg::query::ConfigResponse,
};

pub struct CreditManagerConfigResponse {
    pub red_bank: RedBank,
    pub oracle: Oracle,
    pub health_contract: HealthContract,
    pub max_close_factor: Decimal,
}

impl From<ConfigResponse> for CreditManagerConfigResponse {
    fn from(config: ConfigResponse) -> Self {
        Self {
            red_bank: RedBank::new(Addr::unchecked(config.red_bank)),
            oracle: Oracle::new(Addr::unchecked(config.oracle)),
            health_contract: HealthContract::new(Addr::unchecked(config.health_contract)),
            max_close_factor: config.max_close_factor,
        }
    }
}
