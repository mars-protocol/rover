use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Credit Manger contract is currently not set up in the health contract")]
    NoCreditManager {},
}

pub type ContractResult<T> = Result<T, ContractError>;