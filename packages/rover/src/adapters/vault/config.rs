use cosmwasm_schema::cw_serde;
use cosmwasm_std::Decimal256;
use mars_coin::Coin256;

use crate::error::ContractError;
use crate::error::ContractError::InvalidConfig;

#[cw_serde]
pub struct VaultConfig {
    pub deposit_cap: Coin256,
    pub max_ltv: Decimal256,
    pub liquidation_threshold: Decimal256,
    pub whitelisted: bool,
}

impl VaultConfig {
    pub fn check(&self) -> Result<(), ContractError> {
        let max_ltv_too_big = self.max_ltv > Decimal256::one();
        let lqt_too_big = self.liquidation_threshold > Decimal256::one();
        let max_ltv_bigger_than_lqt = self.max_ltv > self.liquidation_threshold;

        if max_ltv_too_big || lqt_too_big || max_ltv_bigger_than_lqt {
            return Err(InvalidConfig {
                reason: "max ltv or liquidation threshold are invalid".to_string(),
            });
        }
        Ok(())
    }
}
