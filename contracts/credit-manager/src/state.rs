use cosmwasm_std::{Addr, Empty, Uint128};
use cw_storage_plus::{Item, Map};

use rover::adapters::{Oracle, RedBank, VaultPosition};
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
pub const TOTAL_VAULT_SHARES: Map<Addr, Shares> = Map::new("total_vault_shares");

// Temporary state to save variables to be used on reply handling
pub const VAULT_WITHDRAW_TEMP_VAR: Item<String> = Item::new("vault_withdraw_temp_var");
pub const VAULT_REQUEST_TEMP_TOKEN_VAR: Item<String> = Item::new("vault_request_temp_token_var");
pub const VAULT_REQUEST_TEMP_AMOUNT_VAR: Item<Uint128> = Item::new("vault_request_temp_amount_var");
pub const VAULT_UNLOCK_TEMP_VAR: Item<String> = Item::new("vault_unlock_temp_var");
