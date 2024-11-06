use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use mars_owner::Owner;
use std::collections::HashSet;

pub const OWNER: Owner = Owner::new("owner");
pub const LSTS: Item<HashSet<String>> = Item::new("lsts");
pub const DESTINATIONS: Map<String, Addr> = Map::new("destination");
pub const VAULT_DENOM: Item<String> = Item::new("denom");
pub const ORACLE: Item<Addr> = Item::new("oracle");
