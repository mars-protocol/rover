use cosmwasm_std::{CheckedFromRatioError, CheckedMultiplyRatioError, StdError};
use thiserror::Error;

pub type HealthResult<T> = Result<T, HealthError>;

#[derive(Error, Debug, PartialEq)]
pub enum HealthError {
    #[error("{0}")]
    CheckedFromRatioError(#[from] CheckedFromRatioError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

    #[error("{0}")]
    Std(#[from] StdError),
}
