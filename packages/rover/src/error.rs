use cosmwasm_std::{
    CheckedFromRatioError, CheckedMultiplyRatioError, DecimalRangeExceeded, OverflowError,
    StdError, Uint128,
};
use thiserror::Error;

use crate::coins::Coins;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Actions resulted in exceeding maximum allowed loan-to-value")]
    AboveMaxLTV,

    #[error("{0}")]
    CheckedFromRatioError(#[from] CheckedFromRatioError),

    #[error("{0}")]
    CheckedMultiply(#[from] CheckedMultiplyRatioError),

    #[error("{0}")]
    DecimalRangeExceeded(#[from] DecimalRangeExceeded),

    #[error("Callbacks cannot be invoked externally")]
    ExternalInvocation,

    #[error("Extra funds received: {0}")]
    ExtraFundsReceived(Coins),

    #[error("Sent fund mismatch. Expected: {expected:?}, received {received:?}")]
    FundsMismatch {
        expected: Uint128,
        received: Uint128,
    },

    #[error("No coin amount set for action")]
    NoAmount,

    #[error("No debt to repay")]
    NoDebt,

    #[error("Not enough funds for action")]
    NotEnoughFunds,

    #[error("Needed: {needed:?}, Actual {actual:?}")]
    NotEnoughShares { needed: Uint128, actual: Uint128 },

    #[error("{0} does not have a position in this vault")]
    NoPosition(String),

    #[error("Position {0} was not a valid position for this token id in this vault")]
    NoPositionMatch(String),

    #[error("{user:?} is not the owner of {token_id:?}")]
    NotTokenOwner { user: String, token_id: String },

    #[error("{0} is not whitelisted")]
    NotWhitelisted(String),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Reply id: {0} not valid")]
    ReplyIdError(u64),

    #[error("{0}")]
    RequirementsNotMet(String),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{user:?} is not authorized to {action:?}")]
    Unauthorized { user: String, action: String },

    #[error("There is more time left on the lock period for this unlocking position")]
    UnlockNotReady,
}
