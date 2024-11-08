use cosmwasm_std::StdError;
use mars_owner::OwnerError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VaultError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("Destination with id {id} already exists")]
    DestinationAlreadyExists { id: String },
}
