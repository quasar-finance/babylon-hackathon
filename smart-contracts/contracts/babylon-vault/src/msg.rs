use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Timestamp, Uint128};
use mars_owner::{OwnerResponse, OwnerUpdate};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub subdenom: String,
    pub oracle: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // permission-less methods
    Deposit {},
    Withdraw {},
    Claim {},
    // owner methods
    RegisterLst { denom: String },
    UnregisterLst { denom: String },
    SetOracle { oracle: String },
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
pub struct Claim {
    pub amount: Uint128,
    pub expiration: Timestamp,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    Value {},
    #[returns(OwnerResponse)]
    Owner {},
    #[returns(Vec<String>)]
    Lsts {},
    #[returns(String)]
    Denom {},
}

#[cw_serde]
pub enum OracleQueryMsg {
    Price { denom: String },
}
