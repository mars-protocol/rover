use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Decimal, Decimal256, Uint256, Uint64};
use mars_coin::Coin256;

use mars_health::health::Health;

use crate::adapters::vault::{Vault, VaultConfig, VaultPosition, VaultUnchecked};
use crate::traits::ToCoins;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Rover contract-level config
    #[returns(ConfigResponse)]
    Config {},
    /// Configs & deposit caps on vaults
    #[returns(Vec<VaultInfoResponse>)]
    VaultsInfo {
        start_after: Option<VaultUnchecked>,
        limit: Option<u32>,
    },
    /// Whitelisted coins
    #[returns(Vec<String>)]
    AllowedCoins {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// All positions represented by token with value
    #[returns(Positions)]
    Positions { account_id: String },
    /// The health of the account represented by token
    #[returns(HealthResponse)]
    Health { account_id: String },
    /// Enumerate coin balances for all token positions; start_after accepts (account_id, denom)
    #[returns(Vec<CoinBalanceResponseItem>)]
    AllCoinBalances {
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Enumerate debt shares for all token positions; start_after accepts (account_id, denom)
    #[returns(Vec<SharesResponseItem>)]
    AllDebtShares {
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Total debt shares issued for Coin
    #[returns(DebtShares)]
    TotalDebtShares(String),
    /// Enumerate total debt shares for all supported coins; start_after accepts denom string
    #[returns(Vec<DebtShares>)]
    AllTotalDebtShares {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Enumerate all vault positions; start_after accepts (account_id, addr)
    #[returns(Vec<VaultPositionResponseItem>)]
    AllVaultPositions {
        start_after: Option<(String, String)>,
        limit: Option<u32>,
    },
    /// Get total vault coin balance in Rover for vault
    #[returns(Uint256)]
    TotalVaultCoinBalance { vault: VaultUnchecked },
    /// Enumerate all total vault coin balances; start_after accepts vault addr
    #[returns(Vec<VaultWithBalance>)]
    AllTotalVaultCoinBalances {
        start_after: Option<VaultUnchecked>,
        limit: Option<u32>,
    },
    /// Estimate how many LP tokens received in exchange for coins provided for liquidity
    #[returns(Uint256)]
    EstimateProvideLiquidity {
        lp_token_out: String,
        coins_in: Vec<Coin>,
    },
    /// Estimate coins withdrawn if exchanged for LP tokens
    #[returns(Vec<Coin>)]
    EstimateWithdrawLiquidity { lp_token: Coin },
}

#[cw_serde]
pub struct VaultInfoResponse {
    pub vault: VaultUnchecked,
    pub config: VaultConfig,
    /// The amount the vault has been utilized,
    /// denominated in the same denom set in the vault config's deposit cap
    pub utilization: Coin256,
}

#[cw_serde]
pub struct CoinBalanceResponseItem {
    pub account_id: String,
    pub denom: String,
    pub amount: Uint256,
}

#[cw_serde]
pub struct SharesResponseItem {
    pub account_id: String,
    pub denom: String,
    pub shares: Uint256,
}

#[cw_serde]
pub struct DebtShares {
    pub denom: String,
    pub shares: Uint256,
}

#[cw_serde]
pub struct DebtAmount {
    pub denom: String,
    /// number of shares in debt pool
    pub shares: Uint256,
    /// amount of coins
    pub amount: Uint256,
}

impl ToCoins for Vec<DebtAmount> {
    fn to_coins(&self) -> Vec<Coin256> {
        self.iter()
            .map(|d| Coin256 {
                denom: d.denom.to_string(),
                amount: d.amount,
            })
            .collect()
    }
}

#[cw_serde]
pub struct CoinValue {
    pub denom: String,
    pub amount: Uint256,
    pub price: Decimal,
    pub value: Decimal,
}

#[cw_serde]
pub struct Positions {
    pub account_id: String,
    pub deposits: Vec<Coin256>,
    pub debts: Vec<DebtAmount>,
    pub vaults: Vec<VaultPosition>,
}

#[cw_serde]
pub struct VaultPositionResponseItem {
    pub account_id: String,
    pub position: VaultPosition,
}

#[cw_serde]
pub struct VaultWithBalance {
    pub vault: Vault,
    pub balance: Uint256,
}

#[cw_serde]
pub struct VaultPositionValue {
    pub position: VaultPosition,
    pub value: Decimal,
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Option<String>,
    pub proposed_new_owner: Option<String>,
    pub account_nft: Option<String>,
    pub red_bank: String,
    pub oracle: String,
    pub max_close_factor: Decimal256,
    pub max_unlocking_positions: Uint64,
    pub swapper: String,
    pub zapper: String,
}

#[cw_serde]
pub struct HealthResponse {
    pub total_debt_value: Uint256,
    pub total_collateral_value: Uint256,
    pub max_ltv_adjusted_collateral: Uint256,
    pub liquidation_threshold_adjusted_collateral: Uint256,
    pub max_ltv_health_factor: Option<Decimal256>,
    pub liquidation_health_factor: Option<Decimal256>,
    pub liquidatable: bool,
    pub above_max_ltv: bool,
}

impl From<Health> for HealthResponse {
    fn from(h: Health) -> Self {
        Self {
            total_debt_value: h.total_debt_value,
            total_collateral_value: h.total_collateral_value,
            max_ltv_adjusted_collateral: h.max_ltv_adjusted_collateral,
            liquidation_threshold_adjusted_collateral: h.liquidation_threshold_adjusted_collateral,
            max_ltv_health_factor: h.max_ltv_health_factor,
            liquidation_health_factor: h.liquidation_health_factor,
            liquidatable: h.is_liquidatable(),
            above_max_ltv: h.is_above_max_ltv(),
        }
    }
}
