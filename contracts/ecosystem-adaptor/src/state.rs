// Dependencies
use crate::msg::{CellarInfo, DepositDetails, PolytoneInfo};
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Contract state
pub const DEPOSITS: Item<DepositDetails> = Item::new("deposits");
pub const MAIN_VAULT: Item<Addr> = Item::new("main_vault");
pub const CELLAR_INFO : Item<CellarInfo> = Item::new("cellar_info");
pub const POLYTONE_INFO : Item<PolytoneInfo> = Item::new("polytone_info");