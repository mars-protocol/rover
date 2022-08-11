use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::adapters::RedBank;
use crate::Shares;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RepayCalculation {
    pub red_bank: RedBank,
    pub total_debt_shares: Shares,
    pub current_debt: Shares,
    pub shares_to_repay: Shares,
    pub amount_to_repay: Uint128,
}
