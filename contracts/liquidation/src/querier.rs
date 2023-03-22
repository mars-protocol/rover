use cosmwasm_std::{Addr, QuerierWrapper};
use mars_rover::{
    adapters::{
        oracle::Oracle,
        red_bank::RedBank,
        vault::{Vault, VaultConfig},
    },
    error::ContractResult,
    msg::query::{ConfigResponse, Positions, QueryMsg, VaultConfigResponse},
};
use mars_rover_health_types::HealthResult;

use crate::types::CreditManagerConfigResponse;

pub struct LiquidationQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
    credit_manager_addr: &'a Addr,
}

impl<'a> LiquidationQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper, credit_manager_addr: &'a Addr) -> Self {
        Self {
            querier,
            credit_manager_addr,
        }
    }

    pub fn query_credit_manager_config(&self) -> ContractResult<CreditManagerConfigResponse> {
        let config: ConfigResponse = self
            .querier
            .query_wasm_smart(self.credit_manager_addr.to_string(), &QueryMsg::Config {})?;
        Ok(config.into())
    }

    pub fn query_positions(&self, account_id: &str) -> ContractResult<Positions> {
        Ok(self.querier.query_wasm_smart(
            self.credit_manager_addr.to_string(),
            &QueryMsg::Positions {
                account_id: account_id.to_string(),
            },
        )?)
    }

    pub fn query_allowed_coins(&self) -> ContractResult<Vec<String>> {
        let allowed_coins: Vec<String> = self.querier.query_wasm_smart(
            self.credit_manager_addr.to_string(),
            &QueryMsg::AllowedCoins {
                start_after: None,
                limit: Some(u32::MAX),
            },
        )?;
        Ok(allowed_coins)
    }

    pub fn query_vault_config(&self, vault: &Vault) -> ContractResult<VaultConfig> {
        let vault_info: VaultConfigResponse = self.querier.query_wasm_smart(
            self.credit_manager_addr.to_string(),
            &QueryMsg::VaultConfig {
                vault: vault.into(),
            },
        )?;
        Ok(vault_info.config)
    }
}
