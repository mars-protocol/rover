use cosmwasm_std::{Coin, ConversionOverflowError, Uint256};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, JsonSchema)]
pub struct Coin256 {
    pub denom: String,
    pub amount: Uint256,
}

impl Coin256 {
    pub fn new(amount: impl Into<u128>, denom: impl Into<String>) -> Self {
        Coin256 {
            amount: Uint256::from(amount.into()),
            denom: denom.into(),
        }
    }
}

impl From<Coin> for Coin256 {
    fn from(c: Coin) -> Self {
        Coin256::from(&c)
    }
}

impl From<&Coin> for Coin256 {
    fn from(c: &Coin) -> Self {
        Coin256 {
            denom: c.denom.clone(),
            amount: c.amount.into(),
        }
    }
}

impl fmt::Display for Coin256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.amount, self.denom)
    }
}

pub fn coin256(amount: u128, denom: impl Into<String>) -> Coin256 {
    Coin256::new(amount, denom)
}

impl TryFrom<&Coin256> for Coin {
    type Error = ConversionOverflowError;

    fn try_from(c: &Coin256) -> Result<Self, Self::Error> {
        Ok(Self {
            denom: c.denom.clone(),
            amount: c.amount.try_into()?,
        })
    }
}

impl TryFrom<Coin256> for Coin {
    type Error = ConversionOverflowError;

    fn try_from(c: Coin256) -> Result<Self, Self::Error> {
        Coin::try_from(&c)
    }
}
