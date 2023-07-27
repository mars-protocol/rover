use helpers::{max_borrow_prop_test_runner, osmo_atom_1_config};
use mars_rover_health_types::BorrowTarget;

pub mod helpers;

#[test]
fn max_borrow_amount_deposit_renders_healthy_max_ltv() {
    max_borrow_prop_test_runner(2000, &BorrowTarget::Deposit);
}

#[test]
fn max_borrow_amount_wallet_renders_healthy_max_ltv() {
    max_borrow_prop_test_runner(2000, &BorrowTarget::Wallet);
}

#[test]
fn max_borrow_amount_vault_renders_healthy_max_ltv() {
    let osmo_atom_1_config = osmo_atom_1_config();
    max_borrow_prop_test_runner(
        2000,
        &BorrowTarget::Vault {
            address: osmo_atom_1_config.addr.clone(),
        },
    );
}
