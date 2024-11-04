use crate::error::{assert_deposit_funds, assert_withdraw_funds, VaultError};
use crate::msg::{ExecuteMsg, InstantiateMsg, LstInfo, OracleQueryMsg, QueryMsg};
use crate::state::{LSTS, OWNER, VAULT_DENOM};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, BankQuery, Binary, Coin, CustomQuery, Decimal, Deps, DepsMut,
    Env, MessageInfo, Order, QueryRequest, Reply, Response, StdResult, Storage, SubMsg,
    SupplyResponse, Uint128,
};
use cw2::set_contract_version;
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::{
    MsgBurn, MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};
use std::collections::HashMap;

use gauge_interface::*;

const CONTRACT_NAME: &str = "quasar:mock-gauge";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type GaugeResult<T = Response> = Result<T, VaultError>;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> GaugeResult {
    OWNER.initialize(
        deps.storage,
        deps.api,
        mars_owner::OwnerInit::SetInitialOwner { owner: msg.owner },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    
    Ok(Response::new().add_submessage(SubMsg::reply_on_success(msg, CREATE_DENOM_REPLY_ID)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> GaugeResult {
    match reply.id {
        _ => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> GaugeResult {
    match msg {}
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> GaugeResult<Binary> {
    match msg {}
}