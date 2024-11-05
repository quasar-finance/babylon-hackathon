use cosmwasm_std::Decimal;
use cw_storage_plus::Map;
use mars_owner::Owner;

pub const OWNER: Owner = Owner::new("owner");
pub const PRICES: Map<String, Decimal> = Map::new("prices");
