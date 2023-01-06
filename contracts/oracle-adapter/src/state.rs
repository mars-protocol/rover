use cw_storage_plus::{Item, Map};
use mars_owner::Owner;

use mars_rover::adapters::oracle::{Oracle, VaultPricingInfo};

pub const OWNER: Owner = Owner::new("owner");
pub const ORACLE: Item<Oracle> = Item::new("oracle");

/// Map<(Vault Token Denom, Pricing Method)>
pub const VAULT_PRICING_INFO: Map<&str, VaultPricingInfo> = Map::new("vault_coin");
