use crate::adapters::oracle::OracleBase;
use cosmwasm_std::{Addr, Api, Decimal256, StdResult};

pub type OracleAdapterUnchecked = OracleBase<String, Decimal256>;
pub type OracleAdapter = OracleBase<Addr, Decimal256>;

impl OracleAdapterUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<OracleAdapter> {
        Ok(OracleBase::new(api.addr_validate(self.address())?))
    }
}
