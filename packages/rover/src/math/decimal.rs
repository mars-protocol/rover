use cosmwasm_std::{
    CheckedMultiplyRatioError, ConversionOverflowError, Decimal, Decimal256, Fraction, Uint128,
    Uint256,
};
use thiserror::Error;

pub type DecimalMathResult<T> = Result<T, DecimalMathError>;

#[derive(Error, Debug, PartialEq)]
pub enum DecimalMathError {
    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),
}

pub trait MulDecimal {
    fn mul_decimal(&self, decimal: Decimal) -> DecimalMathResult<Uint128>;
    fn mul_decimal_256(&self, decimal: Decimal256) -> DecimalMathResult<Uint128>;
}

impl MulDecimal for Uint128 {
    fn mul_decimal(&self, decimal: Decimal) -> DecimalMathResult<Uint128> {
        Ok(self.checked_multiply_ratio(decimal.numerator(), decimal.denominator())?)
    }

    fn mul_decimal_256(&self, decimal: Decimal256) -> DecimalMathResult<Uint128> {
        let result = Uint256::from(self.u128())
            .checked_multiply_ratio(decimal.numerator(), decimal.denominator())?;
        Ok(result.try_into()?)
    }
}

pub trait DivDecimal {
    fn div_decimal(&self, decimal: Decimal) -> DecimalMathResult<Uint128>;
    fn div_decimal_256(&self, decimal: Decimal256) -> DecimalMathResult<Uint128>;
}

impl DivDecimal for Uint128 {
    fn div_decimal(&self, decimal: Decimal) -> DecimalMathResult<Uint128> {
        Ok(self.checked_multiply_ratio(decimal.denominator(), decimal.numerator())?)
    }

    fn div_decimal_256(&self, decimal: Decimal256) -> DecimalMathResult<Uint128> {
        let result = Uint256::from(self.u128())
            .checked_multiply_ratio(decimal.denominator(), decimal.numerator())?;
        Ok(result.try_into()?)
    }
}
