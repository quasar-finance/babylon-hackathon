use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Uint128};
use mars_owner::{OwnerResponse, OwnerUpdate};
use interfaces::gauge::{GaugeExecuteMsg, GaugeQueryMsg};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

pub type ExecuteMsg = GaugeExecuteMsg;

// #[derive(QueryResponses)]
pub type QueryMsg = GaugeQueryMsg;

