use crate::error::assert_denoms;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{BABYLON_VAULT, ECOSYSTEM_INFO, POLYTONE_INFO};
use crate::AdaptorError;
use cosmwasm_std::{entry_point, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response};

pub type AdaptorResult<T = Response> = Result<T, AdaptorError>;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> AdaptorResult {
    let babylon_vault = deps.api.addr_validate(&msg.babylon_vault)?;
    // todo: perform sanity checks
    BABYLON_VAULT.save(deps.storage, &babylon_vault)?;
    ECOSYSTEM_INFO.save(deps.storage, &msg.ecosystem_info.clone())?;
    POLYTONE_INFO.save(deps.storage, &msg.polytone_info)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("module", "ecosystem_adaptor")
        .add_attribute("babylon_vault", babylon_vault.to_string())
        .add_attribute("ecosystem", msg.ecosystem_info.deposit_ecosystem.clone())
        .add_attribute(
            "deposit_denoms",
            format!("{:?}", msg.ecosystem_info.deposit_denoms), // Formats list of denoms
        )
        .add_attribute(
            "polyton_note_contract",
            msg.polytone_info.polyton_note_contract.clone(),
        ))
}

#[entry_point]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> AdaptorResult {
    match msg {
        ExecuteMsg::Deposit {} => deposit(deps, info),
        ExecuteMsg::Withdraw { amounts } => withdraw(deps, info, amounts),
    }
}

fn deposit(deps: DepsMut, info: MessageInfo) -> AdaptorResult {
    // perform sanity checks
    let babylon_vault = BABYLON_VAULT.load(deps.storage)?;
    if info.sender != babylon_vault {
        return Err(AdaptorError::Unauthorized {});
    }

    assert_denoms(deps.storage, &info.funds)?;

    // Actual steps
    // step 1 : perfrom ibc send funds to other chain
    // step 2 : perfrom ica deposit into vaults on the other chain
    // step 3 : perform action on ica ack to make sure that the deposit went through

    Ok(Response::new().add_attribute(
        "deposit",
        info.funds
            .clone()
            .iter()
            .map(|coin| format!("{}{}", coin.amount, coin.denom))
            .collect::<Vec<String>>()
            .join(", "),
    ))
}

fn withdraw(deps: DepsMut, info: MessageInfo, amounts: Vec<Coin>) -> AdaptorResult {
    // perform sanity checks
    let babylon_vault = BABYLON_VAULT.load(deps.storage)?;
    if info.sender != babylon_vault {
        return Err(AdaptorError::Unauthorized {});
    }

    // Actual steps
    // step 1 : icq to check if adaptor hold desired shares on other chain vault
    // step 2 : ica withdraw on the other chain
    // step 3 : get ica ack and trigger ibc funds back to adaptor
    // step 4 : bank send funds to babylon vault

    // DEMO CODE STARTS
    let send_msg = BankMsg::Send {
        to_address: babylon_vault.to_string(),
        amount: amounts.clone(),
    };
    // DEMO CODE ENDS

    Ok(Response::new().add_message(send_msg).add_attribute(
        "withdraw",
        amounts
            .clone()
            .iter()
            .map(|coin| format!("{}{}", coin.amount, coin.denom))
            .collect::<Vec<String>>()
            .join(", "),
    ))
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> AdaptorResult<Binary> {
    Err(AdaptorError::UnsupportedQuery {})
}
