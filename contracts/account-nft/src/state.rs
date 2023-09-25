use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;
use mars_account_nft_types::nft_config::NftConfig;

pub const CONFIG: Item<NftConfig> = Item::new("config");
pub const NEXT_ID: Item<u64> = Item::new("next_id");

/// Helper marker used during clearing empty accounts. Used only for v1 -> v2 migration.
#[cw_serde]
pub enum ClearingMarker {
    StartAfter(String),
    Finished,
}
pub const MIGRATION_CLEARING_MARKER: Item<ClearingMarker> = Item::new("clearing_marker");
