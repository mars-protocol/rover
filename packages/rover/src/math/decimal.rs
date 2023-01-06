use cosmwasm_std::{
    CheckedMultiplyRatioError, ConversionOverflowError, Decimal256, Fraction, Uint128, Uint256,
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

pub trait MulDecimal256 {
    fn mul_decimal(&self, decimal: Decimal256) -> DecimalMathResult<Uint128>;
}

impl MulDecimal256 for Uint128 {
    fn mul_decimal(&self, decimal: Decimal256) -> DecimalMathResult<Uint128> {
        let result = Uint256::from(self.u128())
            .checked_multiply_ratio(decimal.numerator(), decimal.denominator())?;
        Ok(result.try_into()?)
    }
}

pub trait DivDecimal256 {
    fn div_decimal(&self, decimal: Decimal256) -> DecimalMathResult<Uint128>;
}

impl DivDecimal256 for Uint128 {
    fn div_decimal(&self, decimal: Decimal256) -> DecimalMathResult<Uint128> {
        let result = Uint256::from(self.u128())
            .checked_multiply_ratio(decimal.denominator(), decimal.numerator())?;
        Ok(result.try_into()?)
    }
}
