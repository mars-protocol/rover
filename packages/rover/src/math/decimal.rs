use cosmwasm_std::{CheckedMultiplyRatioError, Decimal256, Fraction, Uint256};

pub trait MulDecimal {
    fn mul_decimal(&self, decimal: Decimal256) -> Result<Uint256, CheckedMultiplyRatioError>;
}

impl MulDecimal for Uint256 {
    fn mul_decimal(&self, decimal: Decimal256) -> Result<Uint256, CheckedMultiplyRatioError> {
        self.checked_multiply_ratio(decimal.numerator(), decimal.denominator())
    }
}

pub trait DivDecimal {
    fn div_decimal(&self, decimal: Decimal256) -> Result<Uint256, CheckedMultiplyRatioError>;
}

impl DivDecimal for Uint256 {
    fn div_decimal(&self, decimal: Decimal256) -> Result<Uint256, CheckedMultiplyRatioError> {
        self.checked_multiply_ratio(decimal.denominator(), decimal.numerator())
    }
}
