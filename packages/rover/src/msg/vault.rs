use cosmwasm_std::{Coin, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Partial compatibility with EIP-4626
/// Balance of LP token should be called via BankQuery::Balance {}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Enters list of `Vec<Coin>` into a vault strategy in exchange for LP tokens
    Deposit,
    /// Withdraw assets in vault by exchanging vault `Coin`
    Withdraw,
    /// Some vaults have lockup periods (typically between 1-14 days). This action sends vault `Coin`
    /// which is locked for vault lockup period and available to `Unlock` after that time has elapsed.
    /// On response, vault sends back `unlocking_position_created` event with attribute `id` representing
    /// the new unlocking tokens position.
    RequestUnlock,
    /// Withdraw assets in vault that have been unlocked for given unlocking position
    Unlock { id: Uint128 },
    /// A privileged action only to be used by Rover. Same as `Withdraw` except it bypasses any lockup period
    /// restrictions on the vault. Used only in the case position is unhealthy and requires immediate liquidation.
    ForceWithdraw,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns `VaultInfo` representing vault requirements, lockup, & vault token denom
    Info,
    /// Returns `Vec<Coin>` representing all the assets would be redeemed for in exchange for LP shares
    /// Used by Rover to calculate vault position values
    PreviewRedeem { shares: Uint128 },
    /// Returns `Vec<UnlockingTokens>` representing the vault `Coin` that this address has requested to unlock
    Unlocking { addr: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VaultInfo {
    /// Assets required to enter vault
    pub assets: Vec<Coin>,
    /// Time in seconds for unlock period
    pub lockup: Option<u64>,
    /// Denom of vault token
    pub token_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnlockingTokens {
    /// Unique identifier representing the unlocking position. Needed for `ExecuteMsg::Unlock {}` call.
    pub id: Uint128,
    /// Number of vault tokens
    pub amount: Uint128,
    /// Absolute time when position unlocks in seconds since the UNIX epoch (00:00:00 on 1970-01-01 UTC)
    pub unlocked_at: Timestamp,
}
