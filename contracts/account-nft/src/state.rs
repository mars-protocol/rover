use cw_storage_plus::Item;
use mars_rover::nft_config::NftConfig;

pub const CONFIG: Item<NftConfig> = Item::new("config");
pub const NEXT_ID: Item<u64> = Item::new("next_id");
