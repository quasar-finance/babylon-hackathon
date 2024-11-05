use cosmwasm_std::StdError;
use mars_owner::OwnerError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AdaptorError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{denom} not found")]
    DenomNotFound { denom: String },

    #[error("invalid funds")]
    InvalidFunds {},

    #[error("unauthorised")]
    Unauthorized {},

    #[error("Insufficient funds for withdrawal")]
    InsufficientFunds {},
}
