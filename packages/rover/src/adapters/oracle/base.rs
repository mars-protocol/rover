use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal256, QuerierWrapper, StdResult, Uint128};
use mars_math::MulDecimal;
use serde::de::DeserializeOwned;

use crate::adapters::oracle::{PriceResponse, QueryMsg};
use crate::error::ContractResult;

#[cw_serde]
pub struct OracleBase<T: Into<String> + Clone, R: Into<Decimal256>> {
    address: T,
    price_response: PhantomData<R>,
}

impl<T: Into<String> + Clone, R: Into<Decimal256>> OracleBase<T, R> {
    pub fn new(address: T) -> Self {
        Self {
            address,
            price_response: PhantomData,
        }
    }

    pub fn address(&self) -> &T {
        &self.address
    }
}

impl<T: Into<String> + Clone, R: DeserializeOwned + Into<Decimal256>> OracleBase<T, R> {
    pub fn query_price(
        &self,
        querier: &QuerierWrapper,
        denom: &str,
    ) -> StdResult<PriceResponse<R>> {
        querier.query_wasm_smart(
            self.address.clone().into(),
            &QueryMsg::Price {
                denom: denom.to_string(),
            },
        )
    }

    pub fn query_total_value(
        &self,
        querier: &QuerierWrapper,
        coins: &[Coin],
    ) -> ContractResult<Uint128> {
        Ok(coins
            .iter()
            .map(|coin| {
                let res = self.query_price(querier, &coin.denom)?;
                Ok(coin.amount.mul_decimal_256(res.price.into())?)
            })
            .collect::<ContractResult<Vec<_>>>()?
            .iter()
            .sum())
    }
}
