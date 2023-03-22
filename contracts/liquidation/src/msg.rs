use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;
use mars_owner::{OwnerResponse, OwnerUpdate};
use mars_rover::{adapters::vault::VaultUnchecked, msg::execute::LiquidateRequest};

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
    /// Pay back debt of a liquidatable rover account for a via liquidating a specific type of the position.
    Liquidate {
        /// The credit account id of the liquidator (sender of the request)
        liquidator_account_id: String,
        /// The credit account id of the one with a liquidation threshold health factor 1 or below
        liquidatee_account_id: String,
        /// The coin they wish to acquire from the liquidatee (amount returned will include the bonus)
        debt_coin: Coin,
        /// Position details to be liquidated
        request: LiquidateRequest<VaultUnchecked>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub credit_manager_addr: Option<String>,
    pub owner_response: OwnerResponse,
}
