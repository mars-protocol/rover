use cosmwasm_schema::cw_serde;

use mars_coin::Coin256;

use crate::adapters::vault::Vault;
use crate::adapters::vault::VaultPositionAmount;

#[cw_serde]
pub struct VaultUnlockingPosition {
    /// Unique identifier representing the unlocking position. Needed for `ExecuteMsg::WithdrawUnlocked {}` call.
    pub id: u64,
    /// Coins that are awaiting to be unlocked (underlying, not vault tokens)
    pub coin: Coin256,
}

#[cw_serde]
pub struct VaultPosition {
    pub vault: Vault,
    pub amount: VaultPositionAmount,
}

#[cw_serde]
pub enum VaultPositionType {
    UNLOCKED,
    LOCKED,
    UNLOCKING,
}
