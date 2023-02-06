use cosmwasm_std::{CheckedFromRatioError, OverflowError, StdError};
use mars_math::CheckedMultiplyFractionError;
use mars_owner::OwnerError;
use thiserror::Error;

pub type HealthResult<T> = Result<T, HealthError>;

#[derive(Error, Debug, PartialEq)]
pub enum HealthError {
    #[error("{0}")]
    CheckedFromRatio(#[from] CheckedFromRatioError),

    #[error("{0}")]
    CheckedMultiplyFraction(#[from] CheckedMultiplyFractionError),

    #[error("The credit manager address has not been set in config")]
    CreditManagerNotSet,

    #[error("{0} was not provided a market to compute health with")]
    MissingMarket(String),

    #[error("{0} was not provided a price to compute health with")]
    MissingPrice(String),

    #[error("{0} was not provided a vault config to compute health with")]
    MissingVaultConfig(String),

    #[error("{0} was not provided vault info to compute health with")]
    MissingVaultInfo(String),

    #[error("{0} was not provided vault coin + base coin values to compute health with")]
    MissingVaultValues(String),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{0}")]
    Std(#[from] StdError),
}
