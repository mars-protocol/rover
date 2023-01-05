use cosmwasm_std::{CheckedMultiplyRatioError, Uint256, Uint512};

pub trait CeilRatio {
    fn multiply_ratio_ceil(
        &self,
        numerator: Uint256,
        denominator: Uint256,
    ) -> Result<Uint256, CheckedMultiplyRatioError>;
}

impl CeilRatio for Uint256 {
    /// Using `checked_multiply_ratio()` results in a rounding down due to the nature of integer math.
    /// This function performs the same math, but rounds up. The is particularly useful in ensuring
    /// safety in certain situations (e.g. calculating what an account owes)
    fn multiply_ratio_ceil(
        &self,
        numerator: Uint256,
        denominator: Uint256,
    ) -> Result<Uint256, CheckedMultiplyRatioError> {
        // Perform the normal multiply ratio.
        // Converts to Uint256 to reduce likeliness of overflow errors
        let new_numerator = self.full_mul(numerator);
        let denom_256 = Uint512::from(denominator);
        let mut result = new_numerator
            .checked_div(denom_256)
            .map_err(|_| CheckedMultiplyRatioError::DivideByZero)?;

        // Check if there's a remainder with that same division.
        // If so, round up (by adding one).
        if !new_numerator
            .checked_rem(denom_256)
            .map_err(|_| CheckedMultiplyRatioError::DivideByZero)?
            .is_zero()
        {
            result += Uint512::one();
        }

        match result.try_into() {
            Ok(ratio) => Ok(ratio),
            Err(_) => Err(CheckedMultiplyRatioError::Overflow),
        }
    }
}
