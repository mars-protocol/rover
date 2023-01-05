use crate::msg::execute::ActionCoin;
use cosmwasm_std::{
    Coin, Decimal, Decimal256, Decimal256RangeExceeded, DecimalRangeExceeded, Uint128, Uint256,
};
use mars_coin::Coin256;

pub trait Stringify {
    fn to_string(&self) -> String;
}

pub trait ToDenoms {
    fn to_denoms(&self) -> Vec<&str>;
}

pub trait ToCoins {
    fn to_coins(&self) -> Vec<Coin256>;
}

pub trait IntoUint128 {
    fn uint128(&self) -> Uint128;
}

// TODO: delete?
impl IntoUint128 for Decimal {
    fn uint128(&self) -> Uint128 {
        *self * Uint128::new(1)
    }
}

// TODO: delete?
pub trait IntoDecimal128 {
    fn to_dec128(&self) -> Result<Decimal, DecimalRangeExceeded>;
}

impl IntoDecimal128 for Uint128 {
    fn to_dec128(&self) -> Result<Decimal, DecimalRangeExceeded> {
        Decimal::from_atomics(*self, 0)
    }
}

impl IntoDecimal128 for u128 {
    fn to_dec128(&self) -> Result<Decimal, DecimalRangeExceeded> {
        Decimal::from_atomics(*self, 0)
    }
}

pub trait IntoDecimal256 {
    fn to_dec256(&self) -> Result<Decimal256, Decimal256RangeExceeded>;
}

impl IntoDecimal256 for Uint256 {
    fn to_dec256(&self) -> Result<Decimal256, Decimal256RangeExceeded> {
        Decimal256::from_atomics(*self, 0)
    }
}

// impl IntoDecimal128 for Decimal256 {
//     fn to_dec128(&self) -> Result<Decimal, DecimalRangeExceeded> {
//         Decimal::checked_from_ratio(self.numerator(), self.denominator())
//             .map_err(|_| DecimalRangeExceeded)
//     }
// }

pub trait FallbackStr {
    fn fallback(&self, fallback: &str) -> String;
}

impl FallbackStr for String {
    fn fallback(&self, fallback: &str) -> String {
        match self {
            s if !s.is_empty() => s.clone(),
            _ => fallback.to_string(),
        }
    }
}

impl ToDenoms for Vec<Coin> {
    fn to_denoms(&self) -> Vec<&str> {
        self.iter().map(|c| c.denom.as_str()).collect()
    }
}

impl ToDenoms for Vec<ActionCoin> {
    fn to_denoms(&self) -> Vec<&str> {
        self.iter().map(|c| c.denom.as_str()).collect()
    }
}
