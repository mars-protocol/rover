use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

pub const CREDIT_MANAGER: Item<Addr> = Item::new("credit_manager");
pub const MAX_VALUE_FOR_BURN: Item<Decimal> = Item::new("max_value_for_burn");
pub const PENDING_OWNER: Item<Addr> = Item::new("pending_owner");
pub const NEXT_ID: Item<u64> = Item::new("next_id");
