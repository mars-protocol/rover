use cosmwasm_std::{Coin, StdResult, Uint128};
use mars_rover::msg::query::DebtAmount;
use mars_rover_health_computer::HealthComputer;
use mars_rover_health_types::SwapKind;
use proptest::strategy::Strategy;
use proptest::test_runner::{Config, TestRunner};

use super::random_health_computer;

pub fn max_swap_prop_test_runner(cases: u32, kind: &SwapKind) {
    let config = Config::with_cases(cases);

    let mut runner = TestRunner::new(config);
    runner
        .run(
            &random_health_computer().prop_filter(
                "For swap we need to ensure 2 available denom params and 1 valid deposit",
                |h| {
                    if h.denoms_data.params.len() < 2 {
                        return false;
                    } else {
                        let from_denom = h.denoms_data.params.keys().next().unwrap();
                        h.positions
                            .deposits
                            .iter()
                            .map(|d| &d.denom)
                            .collect::<Vec<_>>()
                            .contains(&from_denom)
                    }
                },
            ),
            |h| {
                let from_denom = h.denoms_data.params.keys().next().unwrap();
                let to_denom = h.denoms_data.params.keys().nth(1).unwrap();

                let max_swap = h.max_swap_amount_estimate(from_denom, to_denom, kind).unwrap();

                let health_before = h.compute_health().unwrap();
                if health_before.is_above_max_ltv() {
                    assert_eq!(Uint128::zero(), max_swap);
                } else {
                    let h_new = add_swap(&h, from_denom, to_denom, max_swap)?;
                    let health_after = h_new.compute_health().unwrap();

                    // Ensure still healthy
                    assert!(!health_after.is_above_max_ltv());
                }
                Ok(())
            },
        )
        .unwrap();
}

fn add_swap(
    h: &HealthComputer,
    from_denom: &str,
    to_denom: &str,
    amount: Uint128,
) -> StdResult<HealthComputer> {
    let mut new_h = h.clone();

    let from_coin_index =
        new_h.positions.deposits.iter().position(|c| c.denom == from_denom).unwrap();

    let from_coin = new_h.positions.deposits.get_mut(from_coin_index).unwrap();
    let from_price = new_h.denoms_data.prices.get(from_denom).unwrap();
    let to_price = new_h.denoms_data.prices.get(to_denom).unwrap();

    if amount < from_coin.amount {
        from_coin.amount -= amount;
    } else {
        let debt_amount = amount - from_coin.amount;
        from_coin.amount = Uint128::zero();

        if debt_amount > Uint128::zero() {
            new_h.positions.debts.push(DebtAmount {
                denom: from_denom.to_string(),
                shares: debt_amount * Uint128::new(1000),
                amount: debt_amount,
            });
        }
    }

    if from_coin.amount == Uint128::zero() {
        new_h.positions.deposits.remove(from_coin_index);
    }

    let to_coin_amount = amount.mul_floor(from_price / to_price);
    if let Some(to_coin_index) = new_h.positions.deposits.iter().position(|c| c.denom == to_denom) {
        let to_coin = new_h.positions.deposits.get_mut(to_coin_index).unwrap();
        to_coin.amount += to_coin_amount;
    } else {
        new_h.positions.deposits.push(Coin {
            denom: to_denom.to_string(),
            amount: to_coin_amount,
        });
    }

    Ok(new_h)
}
