use crate::adapters::oracle::OracleBase;
use cosmwasm_std::{Addr, Api, Decimal, StdResult};

pub type OracleUnchecked = OracleBase<String, Decimal>;
pub type Oracle = OracleBase<Addr, Decimal>;

impl OracleUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<Oracle> {
        Ok(OracleBase::new(api.addr_validate(self.address())?))
    }
}

impl From<Oracle> for OracleUnchecked {
    fn from(o: Oracle) -> Self {
        OracleUnchecked::new(o.address().into())
    }
}
