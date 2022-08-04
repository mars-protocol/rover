use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CoinPrice {
    pub denom: String,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    pub coins: Vec<CoinPrice>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Meant to simulate price changes for tests. Not available in prod.
    ChangePrice(CoinPrice),
}

// mocked from: https://github.com/mars-protocol/mars-core/blob/master/packages/mars-core/src/oracle.rs#L155
// cw-asset needs to be implemented before we can import
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    AssetPrice { denom: String },
}