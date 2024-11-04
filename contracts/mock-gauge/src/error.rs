use crate::state::{LSTS, VAULT_DENOM};
use cosmwasm_std::{CheckedMultiplyFractionError, Coin, Order, OverflowError, StdError, Storage};
use mars_owner::OwnerError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VaultError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

}