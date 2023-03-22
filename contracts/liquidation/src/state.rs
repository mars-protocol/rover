use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use mars_owner::Owner;
use mars_rover::adapters::health::HealthContract;

pub const OWNER: Owner = Owner::new("owner");
pub const CREDIT_MANAGER: Item<Addr> = Item::new("credit_manager");
