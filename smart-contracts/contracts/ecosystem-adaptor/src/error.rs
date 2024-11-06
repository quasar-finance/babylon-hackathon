use cosmwasm_std::{Coin, StdError, Storage};
use thiserror::Error;

use crate::state::ECOSYSTEM_INFO;

#[derive(Error, Debug, PartialEq)]
pub enum AdaptorError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{denom} not found")]
    DenomNotFound { denom: String },

    #[error("invalid funds")]
    InvalidFunds {},

    #[error("unauthorised")]
    Unauthorized {},

    #[error("Insufficient funds for withdrawal")]
    InsufficientFunds {},

    #[error("No queries supported")]
    UnsupportedQuery {},
}

fn assert_non_empty_funds(funds: &[Coin]) -> Result<(), AdaptorError> {
    if !funds.is_empty() {
        return Err(AdaptorError::InvalidFunds {});
    }

    Ok(())
}

pub fn assert_denoms(storage: &dyn Storage, funds: &[Coin]) -> Result<(), AdaptorError> {
    assert_non_empty_funds(funds)?;

    // Load the array of accepted denoms
    let deposit_denoms = ECOSYSTEM_INFO.load(storage)?.deposit_denoms;

    // Check each denom in the funds array against the accepted denoms
    for coin in funds {
        if !deposit_denoms.contains(&coin.denom) {
            return Err(AdaptorError::DenomNotFound {
                denom: coin.denom.clone(),
            });
        }
    }

    Ok(())
}
