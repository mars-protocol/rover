use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint256;

use crate::adapters::vault::{
    LockingVaultAmount, UnlockingPositions, VaultAmount, VaultPositionAmount,
    VaultUnlockingPosition,
};

#[cw_serde]
pub enum UpdateType {
    Increment(Uint256),
    Decrement(Uint256),
}

#[cw_serde]
pub enum UnlockingChange {
    Add(VaultUnlockingPosition),
    Decrement { id: u64, amount: Uint256 },
}

#[cw_serde]
pub enum VaultPositionUpdate {
    Unlocked(UpdateType),
    Locked(UpdateType),
    Unlocking(UnlockingChange),
}

impl VaultPositionUpdate {
    pub fn default_amount(&self) -> VaultPositionAmount {
        match self {
            VaultPositionUpdate::Unlocked { .. } => {
                VaultPositionAmount::Unlocked(VaultAmount::new(Uint256::zero()))
            }
            _ => VaultPositionAmount::Locking(LockingVaultAmount {
                locked: VaultAmount::new(Uint256::zero()),
                unlocking: UnlockingPositions::new(vec![]),
            }),
        }
    }
}
