use cosmwasm_std::Uint128;
use mars_rover_health_computer::decrement;
use proptest::{prelude::ProptestConfig, prop_assume, test_runner::TestRunner};

use crate::helpers::random_health_computer;

pub mod helpers;

#[test]
fn withdraw_amount_renders_healthy_max_ltv() {
    let config = ProptestConfig {
        cases: 200,
        max_global_rejects: 1000000,
        ..ProptestConfig::default()
    };

    let mut runner = TestRunner::new(config);
    runner
        .run(&random_health_computer(), |h| {
            // Test requires at least one deposit/debt. None case tested in test_max_withdraw.rs
            prop_assume!(!h.positions.deposits.is_empty());
            prop_assume!(!h.positions.debts.is_empty());

            let random_deposit = h.positions.deposits.first().unwrap().clone();
            let params = h.denoms_data.params.get(&random_deposit.denom).unwrap();

            let max_withdraw = h.max_withdraw_amount(&random_deposit.denom).unwrap();
            let health_before = h.compute_health().unwrap();
            if health_before.is_above_max_ltv() && params.credit_manager.whitelisted {
                assert_eq!(Uint128::zero(), max_withdraw);
            } else {
                let h_new = decrement(&h, &random_deposit.denom, max_withdraw)?;
                let health_after = h_new.compute_health().unwrap();

                // If was unhealthy, ensure health did not worsen
                if health_before.is_above_max_ltv() {
                    assert!(
                        health_after.max_ltv_health_factor.unwrap()
                            >= health_before.max_ltv_health_factor.unwrap()
                    )
                } else {
                    // if was healthy, ensure still healthy
                    assert!(!health_after.is_above_max_ltv());

                    // removing even more should not be possible
                    if max_withdraw > Uint128::zero() {
                        if let Ok(next_h) = decrement(&h_new, &random_deposit.denom, Uint128::one())
                        {
                            assert!(next_h.compute_health().unwrap().is_above_max_ltv());
                        }
                    }
                }
            }
            Ok(())
        })
        .unwrap();
}
