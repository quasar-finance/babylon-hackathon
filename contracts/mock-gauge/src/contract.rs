use crate::error::VaultError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Destination, Weight, Weights, DESTINATIONS, OWNER, WEIGHTS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, BankQuery, Binary, Coin, CustomQuery, Decimal, Deps, DepsMut,
    Empty, Env, MessageInfo, Order, QueryRequest, Reply, Response, StdError, StdResult, Storage,
    SubMsg, SupplyResponse, Uint128,
};
use cw2::set_contract_version;
use interfaces::{
    Allocation, GetAllocationResponse, GetAllocationsResponse, GetDestinationsResponse,
};
use quasar_std::quasarlabs::quasarnode::tokenfactory::v1beta1::{
    MsgBurn, MsgCreateDenom, MsgCreateDenomResponse, MsgMint,
};

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

    for dest in msg.destinations {
        DESTINATIONS.save(deps.storage, dest, &Empty::default())?;
    }

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
        interfaces::ExecuteMsg::AddDestination { destination_id } => {
            execute_add_destination(deps, info, destination_id)
        }
        interfaces::ExecuteMsg::Custom(ext) => match ext {
            crate::msg::ExecuteExtensionMsg::Owner(update) => Ok(OWNER.update(deps, info, update)?),
            crate::msg::ExecuteExtensionMsg::UpsertAllocation {
                destination_id,
                amout,
            } => todo!(),
        },
    }
}

pub fn execute_add_destination(
    deps: DepsMut,
    info: MessageInfo,
    destination_id: String,
) -> GaugeResult {
    OWNER.assert_owner(deps.storage, &info.sender)?;

    if DESTINATIONS.has(deps.storage, destination_id.clone()) {
        return Err(VaultError::DestinationAlreadyExists { id: destination_id });
    }

    DESTINATIONS.save(deps.storage, destination_id.clone(), &Empty::default())?;

    Ok(Response::new()
        .add_attribute("action", "add_destination")
        .add_attribute("destination", destination_id))
}

pub fn execute_upsert_allocation(
    deps: DepsMut,
    destination_id: String,
    amount: Uint128,
) -> GaugeResult {
    if !DESTINATIONS.has(deps.storage, destination_id.clone()) {
        DESTINATIONS.save(deps.storage, destination_id.clone(), &Empty::default())?;
    }

    WEIGHTS.add(
        deps.storage,
        Weight {
            destination_id,
            amount,
        },
    )?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> GaugeResult<Binary> {
    match msg {
        interfaces::QueryMsg::GetAllocations {} => Ok(to_json_binary(&query_allocations(deps)?)?),
        interfaces::QueryMsg::GetAllocation { denom } => {
            Ok(to_json_binary(&query_allocation(deps, denom)?)?)
        }
        interfaces::QueryMsg::GetDestinations {} => Ok(to_json_binary(&query_destinations(deps)?)?),
        interfaces::QueryMsg::Custom(_) => unimplemented!(),
    }
}

fn query_allocations(deps: Deps) -> GaugeResult<GetAllocationsResponse> {
    let allocations: Vec<Allocation> = DESTINATIONS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (destination_id, _) = item?;

            let weight = WEIGHTS.get(deps.storage, destination_id.as_str())?;
            let total_weight = WEIGHTS.total(deps.storage)?;
            Ok(Allocation {
                destination_id,
                amount: Decimal::from_ratio(weight.amount, total_weight),
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    Ok(GetAllocationsResponse { allocations })
}

fn query_allocation(deps: Deps, destination_id: String) -> GaugeResult<GetAllocationResponse> {
    let weight = WEIGHTS.get(deps.storage, destination_id.as_str())?;
    let total_weight = WEIGHTS.total(deps.storage)?;

    let allocation = Allocation {
        destination_id,
        amount: Decimal::from_ratio(weight.amount, total_weight),
    };
    let response = GetAllocationResponse { allocation };
    Ok(response)
}

pub fn query_destinations(deps: Deps) -> GaugeResult<GetDestinationsResponse> {
    let destinations: Result<Vec<Destination>, StdError> = DESTINATIONS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| {
            let (destination, _) = item?;
            Ok(destination)
        })
        .collect();

    Ok(GetDestinationsResponse {
        destinations: destinations?,
    })
}
