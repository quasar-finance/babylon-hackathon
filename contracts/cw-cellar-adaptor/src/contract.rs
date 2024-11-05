// Dependencies
use crate::msg::{DepositDetails, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{DEPOSITS, MAIN_VAULT};
use crate::AdaptorError;
use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    Uint128,
};

pub type AdaptorResult<T = Response> = Result<T, AdaptorError>;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> AdaptorResult {
    let main_vault = deps.api.addr_validate(&msg.main_vault)?;
    MAIN_VAULT.save(deps.storage, &main_vault)?;
    DEPOSITS.save(
        deps.storage,
        &DepositDetails {
            amount: Uint128::zero(),
        },
    )?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Execute
#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> AdaptorResult {
    match msg {
        ExecuteMsg::DepositFunds { amount } => deposit_funds(deps, info, amount),
        ExecuteMsg::WithdrawAndSendFunds { amount } => withdraw_and_send_funds(deps, info, amount),
    }
}

fn deposit_funds(deps: DepsMut, info: MessageInfo, amount: Uint128) -> AdaptorResult {
    let main_vault = MAIN_VAULT.load(deps.storage)?;
    if info.sender != main_vault {
        return Err(AdaptorError::Unauthorized {});
    }

    let mut deposit_details = DEPOSITS.load(deps.storage)?;
    deposit_details.amount += amount;
    DEPOSITS.save(deps.storage, &deposit_details)?;

    Ok(Response::new().add_attribute("method", "deposit_funds"))
}

fn withdraw_and_send_funds(deps: DepsMut, info: MessageInfo, amount: Uint128) -> AdaptorResult {
    let main_vault = MAIN_VAULT.load(deps.storage)?;

    if info.sender != main_vault {
        return Err(AdaptorError::Unauthorized {});
    }

    let mut deposit_details = DEPOSITS.load(deps.storage)?;

    if deposit_details.amount < amount {
        return Err(AdaptorError::InsufficientFunds {});
    }

    deposit_details.amount -= amount;
    DEPOSITS.save(deps.storage, &deposit_details)?;

    let send_msg = BankMsg::Send {
        to_address: main_vault.to_string(),
        amount: vec![Coin {
            denom: "ustake".to_string(),
            amount,
        }],
    };

    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("method", "withdraw_and_send_funds"))
}

// Query
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> AdaptorResult<Binary> {
    match msg {
        QueryMsg::GetDepositDetails {} => Ok(to_json_binary(&DEPOSITS.load(deps.storage)?)?),
    }
}
