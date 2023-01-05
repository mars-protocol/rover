use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal256, QuerierWrapper, StdResult};
use mars_outpost::red_bank::Market;
use mars_outpost::{oracle, red_bank};

#[cw_serde]
pub struct OracleAdapterPriceResponse {
    pub denom: String,
    pub price: Decimal256,
}

pub struct MarsQuerier<'a> {
    querier: &'a QuerierWrapper<'a>,
    oracle_addr: &'a Addr,
    red_bank_addr: &'a Addr,
}

impl<'a> MarsQuerier<'a> {
    pub fn new(
        querier: &'a QuerierWrapper,
        oracle_addr: &'a Addr,
        red_bank_addr: &'a Addr,
    ) -> Self {
        MarsQuerier {
            querier,
            oracle_addr,
            red_bank_addr,
        }
    }

    pub fn query_market(&self, denom: &str) -> StdResult<Market> {
        self.querier.query_wasm_smart(
            self.red_bank_addr,
            &red_bank::QueryMsg::Market {
                denom: denom.to_string(),
            },
        )
    }

    pub fn query_price(&self, denom: &str) -> StdResult<Decimal256> {
        let OracleAdapterPriceResponse { price, .. } = self.querier.query_wasm_smart(
            self.oracle_addr,
            &oracle::QueryMsg::Price {
                denom: denom.to_string(),
            },
        )?;
        Ok(price)
    }
}
