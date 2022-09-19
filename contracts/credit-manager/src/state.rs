use cosmwasm_std::{Addr, Decimal, Empty, Uint128};
use cw_storage_plus::{Item, Map};

use rover::adapters::swap::Swapper;
use rover::adapters::{Oracle, RedBank, VaultPosition};

// Contract config
pub const OWNER: Item<Addr> = Item::new("owner");
pub const ACCOUNT_NFT: Item<Addr> = Item::new("account_nft");
pub const ALLOWED_COINS: Map<&str, Empty> = Map::new("allowed_coins");
pub const ALLOWED_VAULTS: Map<&Addr, Empty> = Map::new("allowed_vaults");
pub const RED_BANK: Item<RedBank> = Item::new("red_bank");
pub const ORACLE: Item<Oracle> = Item::new("oracle");
pub const MAX_LIQUIDATION_BONUS: Item<Decimal> = Item::new("max_liquidation_bonus");
pub const MAX_CLOSE_FACTOR: Item<Decimal> = Item::new("max_close_factor");
pub const SWAPPER: Item<Swapper> = Item::new("swapper");

// Positions
pub const COIN_BALANCES: Map<(&str, &str), Uint128> = Map::new("coin_balance"); // Map<(AccountId, Denom), Amount>
pub const DEBT_SHARES: Map<(&str, &str), Uint128> = Map::new("debt_shares"); // Map<(AccountId, Denom), Shares>
pub const TOTAL_DEBT_SHARES: Map<&str, Uint128> = Map::new("total_debt_shares"); // Map<Denom, Shares>
pub const VAULT_POSITIONS: Map<(&str, Addr), VaultPosition> = Map::new("vault_positions"); // Map<(AccountId, VaultAddr), VaultPosition>
