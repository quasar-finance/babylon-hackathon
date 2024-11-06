use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub babylon_vault: String, // Address of the babylon vault
    pub cellar_info: CellarInfo,
    pub polytone_info: PolytoneInfo,
}

#[cw_serde]
pub struct DepositDetails {
    pub amount: Uint128,
    pub denom: String,
}

#[cw_serde]
pub struct CellarInfo {
    pub deposit_denom: String,
    pub deposit_cellar: String,
    pub transfer_channel: String,
    pub connection: String,
    pub return_source_channel: String,
    pub destination_chain_denom: String,
}

#[cw_serde]
pub struct PolytoneInfo {
    pub polyton_note_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    Withdraw { amount: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(DepositDetails)]
    GetDepositDetails {},
}
