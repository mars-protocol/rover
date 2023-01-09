use cosmwasm_std::{ConversionOverflowError, DivideByZeroError, Fraction, OverflowError, Uint128};
use thiserror::Error;

use crate::CheckedMultiplyFractionError::DivideByZero;

// Delete when merged: https://github.com/CosmWasm/cosmwasm/pull/1566

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CheckedMultiplyFractionError {
    #[error("{0}")]
    DivideByZero(#[from] DivideByZeroError),

    #[error("{0}")]
    ConversionOverflow(#[from] ConversionOverflowError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fractional<T>(pub T, pub T);

impl<T: Copy + From<u8> + PartialEq> Fraction<T> for Fractional<T> {
    fn numerator(&self) -> T {
        self.0
    }
    fn denominator(&self) -> T {
        self.1
    }
    fn inv(&self) -> Option<Self> {
        if self.numerator() == 0u8.into() {
            None
        } else {
            Some(Fractional(self.1, self.0))
        }
    }
}

pub trait FractionMath {
    fn checked_mul_floor<F: Fraction<T>, T: Into<Uint128>>(
        self,
        rhs: F,
    ) -> Result<Self, CheckedMultiplyFractionError>
    where
        Self: Sized;

    fn checked_mul_ceil<F: Fraction<T> + Clone, T: Into<Uint128>>(
        self,
        rhs: F,
    ) -> Result<Self, CheckedMultiplyFractionError>
    where
        Self: Sized;

    fn checked_div_floor<F: Fraction<T>, T: Into<Uint128>>(
        self,
        rhs: F,
    ) -> Result<Self, CheckedMultiplyFractionError>
    where
        Self: Sized;
}

impl FractionMath for Uint128 {
    fn checked_mul_floor<F: Fraction<T>, T: Into<Uint128>>(
        self,
        rhs: F,
    ) -> Result<Self, CheckedMultiplyFractionError> {
        let divisor = rhs.denominator().into();
        let res = self
            .full_mul(rhs.numerator().into())
            .checked_div(divisor.into())?;
        Ok(res.try_into()?)
    }

    fn checked_mul_ceil<F: Fraction<T> + Clone, T: Into<Uint128>>(
        self,
        rhs: F,
    ) -> Result<Self, CheckedMultiplyFractionError> {
        let floor_result = self.checked_mul_floor(rhs.clone())?;
        let divisor = rhs.denominator().into();
        let remainder = self
            .full_mul(rhs.numerator().into())
            .checked_rem(divisor.into())?;
        if !remainder.is_zero() {
            Ok(Uint128::one().checked_add(floor_result)?)
        } else {
            Ok(floor_result)
        }
    }

    fn checked_div_floor<F: Fraction<T>, T: Into<Uint128>>(
        self,
        rhs: F,
    ) -> Result<Self, CheckedMultiplyFractionError> {
        let inverted = rhs.inv().ok_or_else(|| {
            DivideByZero(DivideByZeroError {
                operand: self.to_string(),
            })
        })?;
        self.checked_mul_floor(inverted)
    }
}
