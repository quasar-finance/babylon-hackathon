use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use mars_owner::{OwnerResponse, OwnerUpdate};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetPrice { denom: String, price: Decimal },
    UpdateOwner(OwnerUpdate),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Decimal)]
    Price { denom: String },
    #[returns(OwnerResponse)]
    Owner {},
}
