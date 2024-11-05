// Dependencies
use crate::msg::DepositDetails;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Contract state
pub const DEPOSITS: Item<DepositDetails> = Item::new("deposits");
pub const MAIN_VAULT: Item<Addr> = Item::new("main_vault");
