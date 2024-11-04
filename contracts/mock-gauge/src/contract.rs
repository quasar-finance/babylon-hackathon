use crate::error::{VaultError};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Destination, DESTINATIONS, OWNER};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, BankQuery, Binary, Coin, CustomQuery, Decimal, Deps, DepsMut,
    Env, MessageInfo, Order, QueryRequest, Reply, Response, StdResult, Storage, SubMsg,
    SupplyResponse, Uint128,
};
use cw2::set_contract_version;
use interfaces::{Allocation, GetAllocationResponse, GetAllocationsResponse};
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::{
    MsgBurn, MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};
use std::collections::HashMap;


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
    

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> GaugeResult {
    match reply.id {
        _ => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> GaugeResult {
    match msg {
        interfaces::ExecuteMsg::AddDestination { destination_id } => execute_add_destination(deps, info, destination_id),
        interfaces::ExecuteMsg::Custom(_) => unimplemented!(),
    }
}

fn execute_add_destination(deps: DepsMut, info: MessageInfo, destination_id: String) -> GaugeResult {
    // Only owner can add destinations
    OWNER.assert_owner(deps.storage, &info.sender)?;
    
    // Check if destination already exists
    if DESTINATIONS.has(deps.storage, destination_id.clone()) {
        return Err(VaultError::DestinationAlreadyExists { id: destination_id });
    }

    // Create new destination with minimal info
    let destination = Destination {
        id: destination_id.clone(),
        vault_contract: "".to_string(),
        ibc_channel: None,
    };

    // Save destination
    DESTINATIONS.save(deps.storage, destination_id, &destination)?;

    Ok(Response::new().add_attribute("action", "add_destination"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> GaugeResult<Binary> {
    match msg {
        interfaces::QueryMsg::GetAllocations {} => query_allocations(deps),
        interfaces::QueryMsg::GetAllocation { denom } => query_allocation(deps, denom),
        interfaces::QueryMsg::Custom(_) => unimplemented!(),
    }
}

fn query_allocations(deps: Deps) -> GaugeResult<Binary> {
    let allocations: Vec<Allocation> = DESTINATIONS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (id, _) = item?;
            Ok(Allocation {
                destination_id: id,
                amount: Decimal::zero(), // Mock implementation returns 0 allocations
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    let response = GetAllocationsResponse { allocations };
    Ok(to_json_binary(&response)?)
}

fn query_allocation(deps: Deps, denom: String) -> GaugeResult<Binary> {
    // Mock implementation just returns 0 allocation for any denom
    let allocation = Allocation {
        destination_id: denom,
        amount: Decimal::zero(),
    };
    let response = GetAllocationResponse { allocation };
    Ok(to_json_binary(&response)?)
}