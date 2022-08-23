use cosmwasm_std::{Addr, Empty, Uint128};
use cw_storage_plus::{Item, Map};

use rover::adapters::{Oracle, RedBank, Vault, VaultPosition};
use rover::{Denom, NftTokenId, Shares};

// Contract config
pub const OWNER: Item<Addr> = Item::new("owner");
pub const ACCOUNT_NFT: Item<Addr> = Item::new("account_nft");
pub const ALLOWED_COINS: Map<Denom, Empty> = Map::new("allowed_coins");
pub const ALLOWED_VAULTS: Map<&Addr, Empty> = Map::new("allowed_vaults");
pub const RED_BANK: Item<RedBank> = Item::new("red_bank");
pub const ORACLE: Item<Oracle> = Item::new("oracle");

// Positions
pub const COIN_BALANCES: Map<(NftTokenId, Denom), Uint128> = Map::new("coin_balance");
pub const DEBT_SHARES: Map<(NftTokenId, Denom), Shares> = Map::new("debt_shares");
pub const TOTAL_DEBT_SHARES: Map<Denom, Shares> = Map::new("total_debt_shares");
pub const VAULT_POSITIONS: Map<(NftTokenId, Addr), VaultPosition> = Map::new("vault_positions");
pub const TOTAL_VAULT_COIN_BALANCE: Map<Addr, Uint128> = Map::new("total_vault_coin_balance");

// Temporary state to save variables to be used on reply handling
pub const TEMP_VAULT_DEPOSIT_TOKEN_ID: Item<String> = Item::new("temp_vault_deposit_token_id");
pub const TEMP_VAULT_DEPOSIT_VAULT: Item<Vault> = Item::new("temp_vault_deposit_vault");
