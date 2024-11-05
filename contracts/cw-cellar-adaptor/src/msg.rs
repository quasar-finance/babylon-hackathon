// Dependencies
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub main_vault: String, // Address of the main vault
}

#[cw_serde]
pub struct DepositDetails {
    pub amount: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    DepositFunds { amount: Uint128 },
    WithdrawAndSendFunds { amount: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(DepositDetails)]
    GetDepositDetails {},
}
