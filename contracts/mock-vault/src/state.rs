use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use rover::adapters::Oracle;
use rover::msg::vault::UnlockingTokens;
use rover::Shares;

pub const LP_TOKEN_DENOM: Item<String> = Item::new("lp_token_denom");
pub const TOTAL_VAULT_SHARES: Item<Shares> = Item::new("total_vault_shares");
pub const LOCKUP_TIME: Item<Option<u64>> = Item::new("lockup_time");
pub const ASSETS: Map<String, Uint128> = Map::new("assets"); // Denom -> Amount
pub const ORACLE: Item<Oracle> = Item::new("oracle");
pub const UNLOCKING_TOKENS: Map<Addr, Vec<UnlockingTokens>> = Map::new("unlocking_tokens");
pub const NEXT_UNLOCK_ID: Item<Uint128> = Item::new("next_unlock_id");

// Used for mock LP token minting
pub const CHAIN_BANK: Item<Uint128> = Item::new("chain_bank");
