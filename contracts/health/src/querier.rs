use cosmwasm_std::{Addr, QuerierWrapper};
use mars_rover::{
    adapters::{
        oracle::Oracle,
        red_bank::RedBank,
        vault::{Vault, VaultConfig},
    },
    msg::query::{ConfigResponse, Positions, QueryMsg, VaultConfigResponse},
};
use mars_rover_health_types::HealthResult;

pub struct HealthQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
    credit_manager_addr: &'a Addr,
}

impl<'a> HealthQuerier<'a> {
    pub fn new(querier: &'a QuerierWrapper, credit_manager_addr: &'a Addr) -> Self {
        Self {
            querier,
            credit_manager_addr,
        }
    }

    pub fn query_positions(&self, account_id: &str) -> HealthResult<Positions> {
        Ok(self.querier.query_wasm_smart(
            self.credit_manager_addr.to_string(),
            &QueryMsg::Positions {
                account_id: account_id.to_string(),
            },
        )?)
    }

    pub fn query_deps(&self) -> HealthResult<(Oracle, RedBank)> {
        let config: ConfigResponse = self
            .querier
            .query_wasm_smart(self.credit_manager_addr.to_string(), &QueryMsg::Config {})?;
        Ok((
            Oracle::new(Addr::unchecked(config.oracle)),
            RedBank::new(Addr::unchecked(config.red_bank)),
        ))
    }

    pub fn query_vault_config(&self, vault: &Vault) -> HealthResult<VaultConfig> {
        let vault_info: VaultConfigResponse = self.querier.query_wasm_smart(
            self.credit_manager_addr.to_string(),
            &QueryMsg::VaultConfig {
                vault: vault.into(),
            },
        )?;
        Ok(vault_info.config)
    }
}
