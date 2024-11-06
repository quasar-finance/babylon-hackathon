use crate::error::assert_denom;
use crate::msg::{DepositDetails, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ECOSYSTEM_INFO, DEPOSITS, MAIN_VAULT, POLYTONE_INFO};
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
            denom: msg.ecosystem_info.clone().deposit_denom,
        },
    )?;
    ECOSYSTEM_INFO.save(deps.storage, &msg.ecosystem_info.clone())?;
    POLYTONE_INFO.save(deps.storage, &msg.polytone_info)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> AdaptorResult {
    match msg {
        ExecuteMsg::Deposit {} => deposit(deps, info),
        ExecuteMsg::Withdraw { amount } => withdraw(deps, info, amount),
    }
}

fn deposit(deps: DepsMut, info: MessageInfo) -> AdaptorResult {
    // perform sanity checks
    let main_vault = MAIN_VAULT.load(deps.storage)?;
    if info.sender != main_vault {
        return Err(AdaptorError::Unauthorized {});
    }

    assert_denom(deps.storage, &info.funds)?;

    // Actual steps
    // step 1 : perfrom ibc send funds to other chain
    // step 2 : perfrom ica deposit into vaults on the other chain
    // step 3 : perform action on ica ack to make sure that the deposit went through

    // DEMO CODE STARTS
    let mut deposit_details = DEPOSITS.load(deps.storage)?;
    deposit_details.amount += info.funds[0].amount;
    DEPOSITS.save(deps.storage, &deposit_details)?;
    // DEMO CODE ENDS

    Ok(Response::new().add_attribute("method", "deposit_funds"))
}

fn withdraw(deps: DepsMut, info: MessageInfo, amount: Uint128) -> AdaptorResult {
    // perform sanity checks
    let main_vault = MAIN_VAULT.load(deps.storage)?;
    if info.sender != main_vault {
        return Err(AdaptorError::Unauthorized {});
    }

    // Actual steps
    // step 1 : icq to check if adaptor hold desired shares on other chain vault
    // step 2 : ica withdraw on the other chain
    // step 3 : get ica ack and trigger ibc funds back to adaptor
    // step 4 : bank send funds to babylon vault

    // DEMO CODE STARTS
    let mut deposit_details = DEPOSITS.load(deps.storage)?;

    if deposit_details.amount < amount {
        return Err(AdaptorError::InsufficientFunds {});
    }

    deposit_details.amount -= amount;
    DEPOSITS.save(deps.storage, &deposit_details)?;

    let send_msg = BankMsg::Send {
        to_address: main_vault.to_string(),
        amount: vec![Coin {
            denom: deposit_details.denom,
            amount,
        }],
    };
    // DEMO CODE ENDS

    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("withdraw", amount.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> AdaptorResult<Binary> {
    match msg {
        QueryMsg::GetDepositDetails {} => Ok(to_json_binary(&DEPOSITS.load(deps.storage)?)?),
    }
}
