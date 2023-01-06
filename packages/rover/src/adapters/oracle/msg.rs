use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal256};
use mars_owner::OwnerUpdate;

use crate::adapters::oracle::{Oracle, OracleUnchecked};

#[cw_serde]
pub struct InstantiateMsg {
    pub oracle: OracleUnchecked,
    pub vault_pricing: Vec<VaultPricingInfo>,
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig { new_config: ConfigUpdates },
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// If denom is vault coin, will retrieve priceable underlying before querying oracle
    #[returns(PriceResponse)]
    Price { denom: String },

    #[returns(ConfigResponse)]
    Config {},

    #[returns(VaultPricingInfo)]
    PricingInfo { denom: String },

    #[returns(Vec<VaultPricingInfo>)]
    AllPricingInfo {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct PriceResponse {
    pub denom: String,
    pub price: Decimal256,
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Option<String>,
    pub proposed_new_owner: Option<String>,
    pub oracle: Oracle,
}

#[cw_serde]
#[derive(Default)]
pub struct ConfigUpdates {
    pub oracle: Option<OracleUnchecked>,
    pub vault_pricing: Option<Vec<VaultPricingInfo>>,
}

#[cw_serde]
pub struct VaultPricingInfo {
    pub vault_coin_denom: String,
    pub base_denom: String,
    pub addr: Addr,
    pub method: PricingMethod,
}

#[cw_serde]
pub enum PricingMethod {
    PreviewRedeem,
}
