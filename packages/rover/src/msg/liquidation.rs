use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use mars_owner::{OwnerResponse, OwnerUpdate};

use crate::msg::query::DebtAmount;

#[cw_serde]
pub struct InstantiateMsg {
    /// The address with privileged access to update config
    pub owner: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Manages owner role state
    UpdateOwner(OwnerUpdate),
    /// Update contract config constants
    UpdateConfig {
        credit_manager: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    /// Pay back debt of a liquidatable rover account for a via liquidating a specific type of the position.
    #[returns(LiquidationResponse)]
    Liquidation {
        /// The credit account id of the one with a liquidation threshold health factor 1 or below
        liquidatee_account_id: String,
        /// Debt to repay
        debt_coin: Coin,
        /// The coin they wish to acquire from the liquidatee (amount returned will include the bonus)
        request_coin: Coin,
        /// Total liquidatee debt
        liquidatee_debt_coin: DebtAmount,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub credit_manager_addr: Option<String>,
    pub owner_response: OwnerResponse,
}

#[cw_serde]
pub struct LiquidationResponse {
    /// Final debt to repay by a liquidator
    pub debt_coin: Coin,
    /// Request coin amount to give to a liquidator
    pub request_coin: Coin,
}
