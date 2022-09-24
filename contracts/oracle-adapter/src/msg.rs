use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal};
use rover::adapters::{Oracle, OracleUnchecked};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct CoinPrice {
    pub denom: String,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    pub oracle: OracleUnchecked,
    pub vault_pricing: Vec<VaultPricingInfo>,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig { new_config: ConfigUpdates },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(mars_outpost::oracle::PriceResponse)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub owner: Addr,
    pub oracle: Oracle,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct ConfigUpdates {
    pub owner: Option<String>,
    pub oracle: Option<OracleUnchecked>,
    pub vault_pricing: Option<Vec<VaultPricingInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VaultPricingInfo {
    pub denom: String,
    pub addr: Addr,
    pub method: PricingMethod,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PricingMethod {
    PreviewRedeem,
}
