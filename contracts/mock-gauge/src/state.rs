use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use mars_owner::Owner;

pub const OWNER: Owner = Owner::new("owner");

// TODO wrap this in a Destinations struct to formalize innteractions such as id parsing
pub const DESTINATIONS: Map<String,Destination> = Map::new("destinations");

#[cw_serde]
pub struct Destination {
    pub id: String,
    pub vault_contract: String,
    pub ibc_channel: Option<String>
}
