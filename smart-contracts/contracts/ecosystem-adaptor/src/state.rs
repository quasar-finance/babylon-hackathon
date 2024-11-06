use crate::msg::{EcosystemInfo, PolytoneInfo};
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// Contract state
pub const BABYLON_VAULT: Item<Addr> = Item::new("babylon_vault");
pub const ECOSYSTEM_INFO: Item<EcosystemInfo> = Item::new("ecosystem_info");
pub const POLYTONE_INFO: Item<PolytoneInfo> = Item::new("polytone_info");
