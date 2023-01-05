use cosmwasm_std::{CheckedMultiplyRatioError, ConversionOverflowError, StdError};
use mars_rover::error::ContractError as RoverError;
use thiserror::Error;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    RoverError(#[from] RoverError),

    #[error("{0}")]
    CheckedMultiply(#[from] CheckedMultiplyRatioError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("Lockup position {0} not found")]
    LockupPositionNotFound(u64),

    #[error("This vault is not a locking vault, action not allowed")]
    NotLockingVault,

    #[error("Not allowed to perform action")]
    Unauthorized,

    #[error("There is more time left on the lock period")]
    UnlockNotReady,

    #[error("You must request an unlock first")]
    UnlockRequired,

    #[error("Attempting to deposit incorrect denom")]
    WrongDenomSent,

    #[error("Vault token not sent")]
    VaultTokenNotSent,
}
