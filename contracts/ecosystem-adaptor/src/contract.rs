// Dependencies
use crate::error::assert_denom;
use crate::msg::{DepositDetails, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CELLAR_INFO, DEPOSITS, MAIN_VAULT, POLYTONE_INFO};
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
    let main_vault = deps.api.addr_validate(&msg.babylon_vault)?;
    // todo: perform sanity checks
    MAIN_VAULT.save(deps.storage, &main_vault)?;
    DEPOSITS.save(
        deps.storage,
        &DepositDetails {
            amount: Uint128::zero(),
            denom: msg.cellar_info.clone().deposit_denom,
        },
    )?;
    CELLAR_INFO.save(deps.storage, &msg.cellar_info.clone())?;
    POLYTONE_INFO.save(deps.storage, &msg.polytone_info)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

// Execute
#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> AdaptorResult {
    match msg {
        ExecuteMsg::DepositFunds { } => deposit_funds(deps, info),
        ExecuteMsg::WithdrawAndSendFunds { amount } => withdraw_and_send_funds(deps, info, amount),
    }
}

fn deposit_funds(deps: DepsMut, info: MessageInfo) -> AdaptorResult {
    let main_vault = MAIN_VAULT.load(deps.storage)?;
    if info.sender != main_vault {
        return Err(AdaptorError::Unauthorized {});
    }

    if info.funds.len() == 0 {
        return Err(AdaptorError::InsufficientFunds {})
    }

    // check for denom
    assert_denom(deps.storage, &info.funds)?;

    let mut deposit_details = DEPOSITS.load(deps.storage)?;
    deposit_details.amount += info.funds[0].amount;
    DEPOSITS.save(deps.storage, &deposit_details)?;

    // perfrom ica steps here while depositing into another protocol
    // and wait for reply to make sure that the deposit went through
    
    // let deposit = get_polytone_execute_msg_binary();

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

    // do ica withdraw and then use ack from it to trigger IBCing funds back to adaptor
    // todo : do_ica_withdraw_from_vault()
    // then use replies to send back funds on a successful return of funds
    // todo : do_ibc_back_funds()

    let send_msg = BankMsg::Send {
        to_address: main_vault.to_string(),
        amount: vec![Coin {
            denom: deposit_details.denom,
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
