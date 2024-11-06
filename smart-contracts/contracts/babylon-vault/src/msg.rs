use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Timestamp, Uint128};
#[cfg(not(target_arch = "wasm32"))]
use mars_owner::OwnerResponse;
use mars_owner::OwnerUpdate;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub subdenom: String,
    pub oracle: String,
    pub gauge: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // permission-less methods
    Deposit {},
    Withdraw {
        amount: Uint128,
    },
    Claim {},
    // owner methods
    RegisterLst {
        denom: String,
    },
    UnregisterLst {
        denom: String,
    },
    RegisterDestination {
        destination: String,
        adaptor: String,
    },
    UnregisterDestination {
        destination: String,
    },
    SetOracle {
        oracle: String,
    },
    UpdateOwner(OwnerUpdate),
    Rebalance {},
}

#[cw_serde]
pub struct Claim {
    pub amount: Uint128,
    pub expiration: Timestamp,
}

#[cw_serde]
pub struct DestinationInfo {
    pub destination: String,
    pub adaptor: Addr,
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
    #[returns(Vec<DestinationInfo>)]
    Destinations {},
    #[returns(cw20::BalanceResponse)]
    Balance { address: String },
    #[returns(cw20::TokenInfoResponse)]
    TokenInfo {},
    #[returns(cw20::AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub enum OracleQueryMsg {
    Price { denom: String },
}
