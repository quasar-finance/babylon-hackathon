use cosmwasm_std::{Coin, StdError, Storage};
use thiserror::Error;

use crate::state::DEPOSITS;

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
}

fn assert_non_empty_funds(funds: &[Coin]) -> Result<(), AdaptorError> {
    if funds.len() != 1 {
        return Err(AdaptorError::InvalidFunds {});
    }

    Ok(())
}

pub fn assert_denom(storage: &dyn Storage, funds: &[Coin]) -> Result<(), AdaptorError> {
    assert_non_empty_funds(funds)?;

    let deposit_details = DEPOSITS.load(storage)?;
    if deposit_details.denom != funds[0].denom {
        return Err(AdaptorError::DenomNotFound {
            denom: funds[0].denom.clone(),
        });
    }

    Ok(())
}
