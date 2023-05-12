use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use mars_owner::Owner;

use mars_rover::adapters::params::Params;
use mars_rover::adapters::{
    health::HealthContract,
    oracle::Oracle,
    red_bank::RedBank,
    swap::Swapper,
    vault::{VaultConfig, VaultPositionAmount},
    zapper::Zapper,
};

use crate::vault::RequestTempStorage;

// Contract dependencies
// NOTE: Ensure assert_not_contract_in_config() is updated when an external contract is added here
pub const ACCOUNT_NFT: Item<Addr> = Item::new("account_nft");
pub const ORACLE: Item<Oracle> = Item::new("oracle");
pub const RED_BANK: Item<RedBank> = Item::new("red_bank");
pub const SWAPPER: Item<Swapper> = Item::new("swapper");
pub const VAULT_CONFIGS: Map<&Addr, VaultConfig> = Map::new("vault_configs"); // TODO: Remove this
pub const ZAPPER: Item<Zapper> = Item::new("zapper");
pub const HEALTH_CONTRACT: Item<HealthContract> = Item::new("health_contract");
pub const PARAMS: Item<Params> = Item::new("params");

// Config
pub const OWNER: Owner = Owner::new("owner");
// TODO:
//       - Remove VAULT_CONFIGS and replace with query_vault_config(). Get to build. Get tests to pass.
//       - Update build scripts so that it compiles. May need to remove some asset params in instantiate.
//          - Don't sweat too bad because there is a lot we need to update there.
pub const MAX_UNLOCKING_POSITIONS: Item<Uint128> = Item::new("max_unlocking_positions");

// Positions
pub const COIN_BALANCES: Map<(&str, &str), Uint128> = Map::new("coin_balance"); // Map<(AccountId, Denom), Amount>
pub const DEBT_SHARES: Map<(&str, &str), Uint128> = Map::new("debt_shares"); // Map<(AccountId, Denom), Shares>
pub const TOTAL_DEBT_SHARES: Map<&str, Uint128> = Map::new("total_debt_shares"); // Map<Denom, Shares>
pub const LENT_SHARES: Map<(&str, &str), Uint128> = Map::new("lent_shares"); // Map<(AccountId, Denom), Shares>
pub const TOTAL_LENT_SHARES: Map<&str, Uint128> = Map::new("total_lent_shares"); // Map<Denom, Shares>

pub const VAULT_POSITIONS: Map<(&str, Addr), VaultPositionAmount> = Map::new("vault_positions"); // Map<(AccountId, VaultAddr), VaultPositionAmount>

// Temporary state to save variables to be used on reply handling
pub const VAULT_REQUEST_TEMP_STORAGE: Item<RequestTempStorage> =
    Item::new("vault_request_temp_var");
