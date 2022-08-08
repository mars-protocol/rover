use crate::adapters::{Vault, VaultUnchecked};
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, StdResult, Uint128, WasmMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::instantiate::ConfigUpdates;
use crate::Shares;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    //--------------------------------------------------------------------------------------------------
    // Public messages
    //--------------------------------------------------------------------------------------------------
    /// Mints NFT representing a credit account for user. User can have many.
    CreateCreditAccount,
    /// Update user's position on their credit account
    UpdateCreditAccount {
        token_id: String,
        actions: Vec<Action>,
    },

    //--------------------------------------------------------------------------------------------------
    // Privileged messages
    //--------------------------------------------------------------------------------------------------
    /// Update contract config constants
    UpdateConfig { new_config: ConfigUpdates },
    /// Internal actions only callable by the contract itself
    Callback(CallbackMsg),
}

/// The list of actions that users can perform on their positions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// Deposit native coin of specified type and amount. Verifies if the correct amount is sent with transaction.
    Deposit(Coin),
    /// Borrow coin of specified amount from Red Bank
    Borrow(Coin),
    /// Repay coin of specified amount back to Red Bank
    Repay(Coin),
    /// Deposit assets into vault strategy
    VaultDeposit {
        vault: VaultUnchecked,
        assets: Vec<Coin>,
    },
    /// Withdraw assets from vault
    VaultWithdraw {
        vault: VaultUnchecked,
        shares: Shares,
    },
    /// A privileged action only to be used by Rover. Same as `VaultWithdraw` except it bypasses any lockup period
    /// restrictions on the vault. Used only in the case position is unhealthy and requires immediate liquidation.
    VaultForceWithdraw {
        vault: VaultUnchecked,
        shares: Shares,
    },
    /// Requests unlocking of shares for a vault with a required lock period
    VaultRequestUnlock {
        vault: VaultUnchecked,
        shares: Shares,
    },
    /// Withdraws the assets for unlocking position id from vault. Required time must have elapsed.
    VaultUnlock { id: Uint128, vault: VaultUnchecked },
}

/// Internal actions made by the contract with pre-validated inputs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CallbackMsg {
    /// Borrow specified amount of coin from Red Bank;
    /// Increase the token's asset amount and debt shares;
    Borrow { token_id: String, coin: Coin },
    /// Repay coin of specified amount back to Red Bank;
    /// Decrement the token's asset amount and debt shares;
    Repay { token_id: String, coin: Coin },
    /// Calculate the account's max loan-to-value health factor. If above 1,
    /// emits a `position_changed` event. If 1 or below, raises an error.
    AssertBelowMaxLTV { token_id: String },
    /// Adds list of assets to a vault strategy
    VaultDeposit {
        token_id: String,
        vault: Vault,
        coins: Vec<Coin>,
    },
    /// Exchanges vault LP shares for assets
    VaultWithdraw {
        token_id: String,
        vault: Vault,
        shares: Shares,
    },
    /// Used only for liquidations
    VaultForceWithdraw {
        token_id: String,
        vault: Vault,
        shares: Shares,
    },
    /// Requests unlocking of shares for a vault with a lock period
    VaultRequestUnlock {
        token_id: String,
        vault: Vault,
        shares: Uint128,
    },
    /// Withdraws assets from vault for a locked position having a lockup period that has been fulfilled
    VaultWithdrawUnlocked {
        token_id: String,
        vault: Vault,
        position_id: Uint128,
    },
}

impl CallbackMsg {
    pub fn into_cosmos_msg(&self, contract_addr: &Addr) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&ExecuteMsg::Callback(self.clone()))?,
            funds: vec![],
        }))
    }
}
