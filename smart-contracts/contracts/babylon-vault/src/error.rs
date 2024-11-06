use crate::state::LSTS;
use cosmwasm_std::{CheckedMultiplyFractionError, Coin, OverflowError, StdError, Storage};
use cw20_base::ContractError as Cw20Error;
use mars_owner::OwnerError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VaultError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{0}")]
    CheckedMultiply(#[from] CheckedMultiplyFractionError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{denom} not found")]
    DenomNotFound { denom: String },

    #[error("Destination {destination} not found")]
    DestinationNotFound { destination: String },

    #[error("invalid funds")]
    InvalidFunds {},

    #[error("{0}")]
    Cw20(#[from] Cw20Error),

    #[error("This message does no accept funds")]
    NonPayable {},
}

fn assert_non_empty_funds(funds: &[Coin]) -> Result<(), VaultError> {
    if funds.len() != 1 {
        return Err(VaultError::InvalidFunds {});
    }

    Ok(())
}

fn assert_empty_funds(funds: &[Coin]) -> Result<(), VaultError> {
    if funds.is_empty() {
        return Err(VaultError::InvalidFunds {});
    }

    Ok(())
}

pub fn assert_deposit_funds(storage: &dyn Storage, funds: &[Coin]) -> Result<(), VaultError> {
    assert_non_empty_funds(funds)?;

    let lsts = LSTS.load(storage)?;
    if !lsts.contains(&funds[0].denom) {
        return Err(VaultError::DenomNotFound {
            denom: funds[0].denom.clone(),
        });
    }

    Ok(())
}

pub fn assert_withdraw_funds(_storage: &dyn Storage, funds: &[Coin]) -> Result<(), VaultError> {
    assert_empty_funds(funds)?;
    Ok(())
}
