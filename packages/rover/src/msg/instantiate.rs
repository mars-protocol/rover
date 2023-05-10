use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::adapters::params::ParamsUnchecked;
use crate::{
    adapters::{
        health::HealthContractUnchecked,
        oracle::OracleUnchecked,
        red_bank::RedBankUnchecked,
        swap::SwapperUnchecked,
        vault::{VaultConfig, VaultUnchecked},
        zapper::ZapperUnchecked,
    },
    traits::Stringify,
};

#[cw_serde]
pub struct InstantiateMsg {
    /// The address with privileged access to update config
    pub owner: String,
    /// Whitelisted coin denoms approved by governance
    pub allowed_coins: Vec<String>, // TODO: Remove this
    /// Vaults approved by governance that implement credit manager's vault interface
    /// Includes a deposit cap that enforces a TLV limit for risk mitigation
    pub vault_configs: Vec<VaultInstantiateConfig>, // TODO: Remove this
    /// The Mars Protocol money market contract where we borrow assets from
    pub red_bank: RedBankUnchecked,
    /// The Mars Protocol oracle contract. We read prices of assets here.
    pub oracle: OracleUnchecked,
    /// The maximum number of unlocking positions an account can have simultaneously
    /// Note: As health checking requires looping through each, this number must not be too large.
    ///       If so, having too many could prevent the account from being liquidated due to gas constraints.
    pub max_unlocking_positions: Uint128,
    /// Helper contract for making swaps
    pub swapper: SwapperUnchecked,
    /// Helper contract for adding/removing liquidity
    pub zapper: ZapperUnchecked,
    /// Helper contract for calculating health factor
    pub health_contract: HealthContractUnchecked,
    /// Contract that stores asset and vault params
    pub params: ParamsUnchecked,
}

#[cw_serde]
pub struct VaultInstantiateConfig {
    pub vault: VaultUnchecked,
    pub config: VaultConfig,
}

impl Stringify for Vec<VaultInstantiateConfig> {
    fn to_string(&self) -> String {
        self.iter()
            .map(|v| {
                format!(
                    "addr: {}, deposit_cap: {}, max_ltv: {}, liquidation_threshold: {}, whitelisted: {}",
                    v.vault.address,
                    v.config.deposit_cap,
                    v.config.max_ltv,
                    v.config.liquidation_threshold,
                    v.config.whitelisted
                )
            })
            .collect::<Vec<String>>()
            .join(" :: ")
    }
}

/// Used when you want to update fields on Instantiate config
#[cw_serde]
#[derive(Default)]
pub struct ConfigUpdates {
    pub account_nft: Option<String>,
    pub allowed_coins: Option<Vec<String>>,
    pub vault_configs: Option<Vec<VaultInstantiateConfig>>, // TODO: Remove this
    pub oracle: Option<OracleUnchecked>,
    pub red_bank: Option<RedBankUnchecked>,
    pub max_unlocking_positions: Option<Uint128>,
    pub swapper: Option<SwapperUnchecked>,
    pub zapper: Option<ZapperUnchecked>,
    pub health_contract: Option<HealthContractUnchecked>,
}
