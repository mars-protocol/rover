use std::{collections::HashMap, fmt};

use cosmwasm_std::{Addr, Decimal256, Fraction, QuerierWrapper, Uint256};

use mars_coin::Coin256;
use mars_outpost::red_bank::Market;

use crate::error::HealthResult;
use crate::query::MarsQuerier;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub denom: String,
    pub price: Decimal256,
    pub collateral_amount: Uint256,
    pub debt_amount: Uint256,
    pub max_ltv: Decimal256,
    pub liquidation_threshold: Decimal256,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Health {
    /// The sum of the value of all debts
    pub total_debt_value: Uint256,
    /// The sum of the value of all collaterals
    pub total_collateral_value: Uint256,
    /// The sum of the value of all colletarals adjusted by their Max LTV
    pub max_ltv_adjusted_collateral: Uint256,
    /// The sum of the value of all colletarals adjusted by their Liquidation Threshold
    pub liquidation_threshold_adjusted_collateral: Uint256,
    /// The sum of the value of all collaterals multiplied by their max LTV, over the total value of debt
    pub max_ltv_health_factor: Option<Decimal256>,
    /// The sum of the value of all collaterals multiplied by their liquidation threshold over the total value of debt
    pub liquidation_health_factor: Option<Decimal256>,
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(total_debt_value: {}, total_collateral_value: {},  max_ltv_adjusted_collateral: {}, lqdt_threshold_adjusted_collateral: {}, max_ltv_health_factor: {}, liquidation_health_factor: {})",
            self.total_debt_value,
            self.total_collateral_value,
            self.max_ltv_adjusted_collateral,
            self.liquidation_threshold_adjusted_collateral,
            self.max_ltv_health_factor.map_or("n/a".to_string(), |x| x.to_string()),
            self.liquidation_health_factor.map_or("n/a".to_string(), |x| x.to_string())
        )
    }
}

impl Health {
    /// Compute the health from coins (collateral and debt)
    pub fn compute_health_from_coins(
        querier: &QuerierWrapper,
        oracle_addr: &Addr,
        red_bank_addr: &Addr,
        collateral: &[Coin256],
        debt: &[Coin256],
    ) -> HealthResult<Health> {
        let querier = MarsQuerier::new(querier, oracle_addr, red_bank_addr);
        let positions = Self::positions_from_coins(&querier, collateral, debt)?;

        Self::compute_health(&positions.into_values().collect::<Vec<_>>())
    }

    /// Compute the health for a Position
    pub fn compute_health(positions: &[Position]) -> HealthResult<Health> {
        let mut health = positions.iter().try_fold::<_, _, HealthResult<Health>>(
            Health::default(),
            |mut h, p| {
                let collateral_value = p
                    .collateral_amount
                    .checked_multiply_ratio(p.price.numerator(), p.price.denominator())?;
                h.total_debt_value += p
                    .debt_amount
                    .checked_multiply_ratio(p.price.numerator(), p.price.denominator())?;
                h.total_collateral_value += collateral_value;
                h.max_ltv_adjusted_collateral += collateral_value
                    .checked_multiply_ratio(p.max_ltv.numerator(), p.max_ltv.denominator())?;
                h.liquidation_threshold_adjusted_collateral += collateral_value
                    .checked_multiply_ratio(
                        p.liquidation_threshold.numerator(),
                        p.liquidation_threshold.denominator(),
                    )?;
                Ok(h)
            },
        )?;

        // If there aren't any debts a health factor can't be computed (divide by zero)
        if !health.total_debt_value.is_zero() {
            health.max_ltv_health_factor = Some(Decimal256::checked_from_ratio(
                health.max_ltv_adjusted_collateral,
                health.total_debt_value,
            )?);
            health.liquidation_health_factor = Some(Decimal256::checked_from_ratio(
                health.liquidation_threshold_adjusted_collateral,
                health.total_debt_value,
            )?);
        }

        Ok(health)
    }

    #[inline]
    pub fn is_liquidatable(&self) -> bool {
        self.liquidation_health_factor
            .map_or(false, |hf| hf < Decimal256::one())
    }

    #[inline]
    pub fn is_above_max_ltv(&self) -> bool {
        self.max_ltv_health_factor
            .map_or(false, |hf| hf < Decimal256::one())
    }

    /// Convert a collection of coins (Collateral and debts) to a map of `Position`
    pub fn positions_from_coins(
        querier: &MarsQuerier,
        collateral: &[Coin256],
        debt: &[Coin256],
    ) -> HealthResult<HashMap<String, Position>> {
        let mut positions: HashMap<String, Position> = HashMap::new();

        collateral.iter().try_for_each(|c| -> HealthResult<_> {
            match positions.get_mut(&c.denom) {
                Some(p) => {
                    p.collateral_amount += c.amount;
                }
                None => {
                    let Market {
                        max_loan_to_value,
                        liquidation_threshold,
                        ..
                    } = querier.query_market(&c.denom)?;

                    positions.insert(
                        c.denom.clone(),
                        Position {
                            denom: c.denom.clone(),
                            collateral_amount: c.amount,
                            debt_amount: Uint256::zero(),
                            price: querier.query_price(&c.denom)?,
                            max_ltv: max_loan_to_value.into(),
                            liquidation_threshold: liquidation_threshold.into(),
                        },
                    );
                }
            }
            Ok(())
        })?;

        debt.iter().try_for_each(|d| -> HealthResult<_> {
            match positions.get_mut(&d.denom) {
                Some(p) => {
                    p.debt_amount += d.amount;
                }
                None => {
                    let Market {
                        max_loan_to_value,
                        liquidation_threshold,
                        ..
                    } = querier.query_market(&d.denom)?;

                    positions.insert(
                        d.denom.clone(),
                        Position {
                            denom: d.denom.clone(),
                            collateral_amount: Uint256::zero(),
                            debt_amount: d.amount,
                            price: querier.query_price(&d.denom)?,
                            max_ltv: max_loan_to_value.into(),
                            liquidation_threshold: liquidation_threshold.into(),
                        },
                    );
                }
            }
            Ok(())
        })?;
        Ok(positions)
    }
}
