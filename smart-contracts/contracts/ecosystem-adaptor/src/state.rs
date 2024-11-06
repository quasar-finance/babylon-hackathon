use crate::msg::{EcosystemInfo, DepositDetails, PolytoneInfo};
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Contract state
// DEMO CODE STARTS
pub const DEPOSITS: Item<DepositDetails> = Item::new("deposits");
// DEMO CODE ENDS
pub const MAIN_VAULT: Item<Addr> = Item::new("main_vault");
pub const ECOSYSTEM_INFO: Item<EcosystemInfo> = Item::new("ecosystem_info");
pub const POLYTONE_INFO: Item<PolytoneInfo> = Item::new("polytone_info");
