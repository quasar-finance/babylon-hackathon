use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Uint128};
use interfaces::gauge::{ExecuteMsg as GaugeExecuteMsg, QueryMsg as GaugeQueryMsg};
use mars_owner::{OwnerResponse, OwnerUpdate};

use crate::state::Destination;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub destinations: Vec<Destination>
}

pub type ExecuteMsg = GaugeExecuteMsg<ExtensionMsg>;

#[cw_serde]
pub enum ExtensionMsg {
    Owner(OwnerUpdate)
}

// #[derive(QueryResponses)]
pub type QueryMsg = GaugeQueryMsg;
