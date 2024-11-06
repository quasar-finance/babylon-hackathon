use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub babylon_vault: String, // Address of the babylon vault
    pub ecosystem_info: EcosystemInfo,
    pub polytone_info: PolytoneInfo,
}

#[cw_serde]
pub struct EcosystemInfo {
    pub deposit_denoms: Vec<String>,
    pub deposit_ecosystem: String,
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
    Withdraw { amounts: Vec<Coin> },
}

// DEMO CODE STARTS
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
// DEMO CODE ENDS
