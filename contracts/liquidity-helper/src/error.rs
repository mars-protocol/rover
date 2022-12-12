use cosmwasm_std::{OverflowError, StdError};
use cw_dex::CwDexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CwDexError(#[from] CwDexError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{0}")]
    Generic(String),
}

impl From<&str> for ContractError {
    fn from(msg: &str) -> Self {
        ContractError::Generic(msg.to_string())
    }
}

impl From<String> for ContractError {
    fn from(msg: String) -> Self {
        ContractError::Generic(msg)
    }
}
