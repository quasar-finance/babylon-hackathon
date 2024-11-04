use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Empty};

#[cw_serde]
pub enum ExecuteMsg<T = Empty> {
    AddDestination { destination_id: String },
    Custom(T),
}

#[cw_serde]
pub enum QueryMsg<T = Empty> {
    GetAllocations {},
    GetAllocation { denom: String },
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
/// Allocation is the
pub struct Allocation {
    pub destination_id: String,
    pub amount: Decimal,
}
