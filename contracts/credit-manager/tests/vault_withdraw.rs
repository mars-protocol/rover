use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use mock_vault::contract::STARTING_SHARES;
use rover::msg::execute::Action::{Deposit, VaultDeposit, VaultWithdraw};

use crate::helpers::{get_asset, AccountToFund, CoinInfo, MockEnv, VaultTestInfo};

pub mod helpers;

#[test]
fn test_withdraw_with_unlocked_vault() {
    let uatom = CoinInfo {
        denom: "uatom".to_string(),
        price: Decimal::from_atomics(100u128, 0).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(8u128, 1).unwrap(),
    };

    let uosmo = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(16u128, 0).unwrap(),
        max_ltv: Decimal::from_atomics(6u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(7u128, 1).unwrap(),
    };

    let leverage_vault = VaultTestInfo {
        lp_token_denom: "uleverage".to_string(),
        lockup: None,
        unlock_request_queue: None,
        asset_denoms: vec!["uatom".to_string(), "uosmo".to_string()],
    };

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uatom.clone(), uosmo.clone()])
        .allowed_vaults(&[leverage_vault.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![Coin::new(300u128, "uatom"), Coin::new(500u128, "uosmo")],
        })
        .build()
        .unwrap();

    let vault = mock.get_vault(&leverage_vault);
    let token_id = mock.create_credit_account(&user).unwrap();

    mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Deposit(Coin {
                denom: uatom.denom,
                amount: Uint128::from(200u128),
            }),
            Deposit(Coin {
                denom: uosmo.denom,
                amount: Uint128::from(400u128),
            }),
            VaultDeposit {
                vault: vault.clone(),
                assets: vec![Coin::new(23u128, "uatom"), Coin::new(120u128, "uosmo")],
            },
            VaultWithdraw {
                vault,
                shares: STARTING_SHARES,
            },
        ],
        &[Coin::new(200u128, "uatom"), Coin::new(400u128, "uosmo")],
    )
    .unwrap();

    // Assert token's updated position
    let res = mock.query_position(&token_id);
    // assert_eq!(res.vault_positions.len(), 0); TODO: Add purge callback
    let atom = get_asset("uatom", &res.coins);
    assert_eq!(atom.amount, Uint128::from(200u128));
    let osmo = get_asset("uosmo", &res.coins);
    assert_eq!(osmo.amount, Uint128::from(400u128));

    // Assert Rover indeed has those on hand in the bank
    let atom = mock.query_balance(&mock.rover, "uatom");
    assert_eq!(atom.amount, Uint128::from(200u128));
    let osmo = mock.query_balance(&mock.rover, "uosmo");
    assert_eq!(osmo.amount, Uint128::from(400u128));

    // Assert Rover does not have the vault tokens anymore
    let lp_balance = mock.query_balance(&mock.rover, &leverage_vault.lp_token_denom);
    assert_eq!(Uint128::zero(), lp_balance.amount);
}

// test withdraw works on uneven numbers. Is vault supposed to round down? e.g. 23 / 2
// Separate actions (two transactions), versus one big one (deposit + withdraw in same)
// test_withdraw_with_unlocked_vault
// test only token_owner can do a vault withdraw
// test_failure_when_withdrawing_when_no_unlocked_positions_available
// assert vault is whitelisted
// test if no unlocked but only unlocking ones that are ready to be withdrawn
