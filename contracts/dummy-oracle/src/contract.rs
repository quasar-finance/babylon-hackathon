use crate::error::VaultError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{PRICES, OWNER};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Decimal, Deps, DepsMut,
    Env, MessageInfo, Response, 
};
use cw2::set_contract_version;


const CONTRACT_NAME: &str = "quasar:dummy-oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type VaultResult<T = Response> = Result<T, VaultError>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> VaultResult {
    OWNER.initialize(
        deps.storage,
        deps.api,
        mars_owner::OwnerInit::SetInitialOwner { owner: msg.owner },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecuteMsg) -> VaultResult {
    match msg {
        ExecuteMsg::SetPrice{denom, price} => set_price(deps, info, denom, price),
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
    }
}

fn set_price(deps: DepsMut, info: MessageInfo, denom: String, price: Decimal) -> VaultResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;
    PRICES.save(deps.storage, denom, &price)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> VaultResult<Binary> {
    match msg {
        QueryMsg::Owner {} => Ok(to_json_binary(&OWNER.query(deps.storage)?)?),
        QueryMsg::Price { denom } => Ok(to_json_binary(&query_price(deps, denom)?)?),
    }
}

fn query_price(deps: Deps, denom: String) -> VaultResult<Decimal> {
    let price = PRICES.may_load(deps.storage, denom.clone())?;
    if let Some(price) = price {
        return Ok(price);
    }
    Err(VaultError::DenomNotFound{ denom })
}
