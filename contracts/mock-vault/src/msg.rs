use cosmwasm_std::Uint128;
use rover::adapters::OracleUnchecked;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Remaining messages in packages/rover/msg/vault
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    /// Denom for vault LP share token
    pub lp_token_denom: String,
    /// cw-multi-test does not yet have the ability to mint sdk coins. For this reason,
    /// this contract expects to be pre-funded with lp tokens and it will simulate the mint.
    pub pre_funded_amount: Uint128,
    /// Denoms for assets in vault
    pub asset_denoms: Vec<String>,
    /// Time in seconds for unlock period
    pub lockup: Option<u64>,
    pub oracle: OracleUnchecked,
}
