use cosmwasm_std::{Addr, Api, Decimal256, StdResult};

use crate::adapters::oracle::OracleBase;

pub type OracleAdapterUnchecked = OracleBase<String, Decimal256>;
pub type OracleAdapter = OracleBase<Addr, Decimal256>;

impl OracleAdapterUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<OracleAdapter> {
        Ok(OracleBase::new(api.addr_validate(self.address())?))
    }
}

impl From<OracleAdapter> for OracleAdapterUnchecked {
    fn from(o: OracleAdapter) -> Self {
        OracleAdapterUnchecked::new(o.address().into())
    }
}
