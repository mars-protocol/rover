use cosmwasm_std::{OverflowError, StdError, Uint128};
use cw_dex::CwDexError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("{0}")]
    CwDexError(#[from] CwDexError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient LP tokens. Expected a minimum of {expected} but got {received}")]
    InsufficientLpTokens {
        expected: Uint128,
        received: Uint128,
    },
}
