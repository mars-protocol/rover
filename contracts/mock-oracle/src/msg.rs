use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

use mars_rover::adapters::oracle::PriceResponse;

#[cw_serde]
pub struct CoinPrice {
    pub denom: String,
    pub price: Decimal,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub prices: Vec<CoinPrice>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Meant to simulate price changes for tests. Not available in prod.
    ChangePrice(CoinPrice),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(PriceResponse)]
    Price { denom: String },
}
