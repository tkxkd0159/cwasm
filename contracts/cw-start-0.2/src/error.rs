use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized { owner: String },

    #[error("Insufficient balance to do this action")]
    InsufficientFunds {},

    #[error("invalid contracat name (expected: {expected:?}, actual: {actual:?})")]
    InvalidContract { expected: String, actual: String },

    #[error("invalid contracat version: (actual: {version:?}, expected: < 0.2.0)")]
    InvalidContractVersion { version: String },
}
