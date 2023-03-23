use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api, Coin, QuerierWrapper, StdResult};

use crate::msg::{
    liquidation::{LiquidationResponse, QueryMsg},
    query::DebtAmount,
};

#[cw_serde]
pub struct LiquidationContractBase<T>(T);

impl<T> LiquidationContractBase<T> {
    pub fn new(address: T) -> LiquidationContractBase<T> {
        LiquidationContractBase(address)
    }

    pub fn address(&self) -> &T {
        &self.0
    }
}

pub type LiquidationContractUnchecked = LiquidationContractBase<String>;
pub type LiquidationContract = LiquidationContractBase<Addr>;

impl From<LiquidationContract> for LiquidationContractUnchecked {
    fn from(health: LiquidationContract) -> Self {
        Self(health.address().to_string())
    }
}

impl LiquidationContractUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<LiquidationContract> {
        Ok(LiquidationContractBase::new(api.addr_validate(self.address())?))
    }
}

impl LiquidationContract {
    pub fn query_liquidation(
        &self,
        querier: &QuerierWrapper,
        liquidatee_account_id: String,
        debt_coin: Coin,
        request_coin: Coin,
        liquidatee_debt_coin: DebtAmount,
    ) -> StdResult<LiquidationResponse> {
        querier.query_wasm_smart(
            self.address().to_string(),
            &QueryMsg::Liquidation {
                liquidatee_account_id,
                debt_coin,
                request_coin,
                liquidatee_debt_coin,
            },
        )
    }
}
