use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use interfaces::gauge::{ExecuteMsg as GaugeExecuteMsg, QueryMsg as GaugeQueryMsg};
use mars_owner::OwnerUpdate;

use crate::state::Destination;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub destinations: Vec<Destination>,
}

pub type ExecuteMsg = GaugeExecuteMsg<ExecuteExtensionMsg>;

#[cw_serde]
pub enum ExecuteExtensionMsg {
    Owner(OwnerUpdate),
    UpsertAllocation {
        destination_id: String,
        amount: Uint128,
    },
}

pub type QueryMsg = GaugeQueryMsg;
