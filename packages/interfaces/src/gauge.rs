use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Empty};

#[cw_serde]
pub enum ExecuteMsg<T = Empty> {
    AddDestination { destination_id: String },
    Custom(T),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<T = Empty> {
    #[returns(GetAllocationsResponse)]
    GetAllocations {},
    #[returns(GetAllocationResponse)]
    GetAllocation { denom: String },
    #[returns(Empty)]
    Custom(T),
}

#[cw_serde]
pub struct GetAllocationsResponse {
    allocations: Vec<Allocation>,
}

#[cw_serde]
pub struct GetAllocationResponse {
    allocation: Allocation,
}

#[cw_serde]
pub struct Allocation {
    pub destination_id: String,
    pub amount: Decimal,
}

