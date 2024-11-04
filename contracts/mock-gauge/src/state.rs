use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use mars_owner::Owner;

pub const OWNER: Owner = Owner::new("owner");

// TODO wrap this in a Destinations struct to formalize innteractions such as id parsing
pub const DESTINATIONS: Map<String,Destination> = Map::new("destinations");

pub struct Destination {
    id: String,
    vault_contract: String,
    ibc_channel: Option<String>
}
