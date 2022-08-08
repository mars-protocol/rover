use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use mock_vault::contract::STARTING_SHARES;
use rover::msg::execute::Action::{Deposit, VaultDeposit, VaultRequestUnlock};

use crate::helpers::{AccountToFund, CoinInfo, MockEnv, VaultTestInfo};

pub mod helpers;

#[test]
fn test_request_unlocked() {
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
        lockup: Some(1_209_600),            // 14 days
        unlock_request_queue: Some(86_400), // 1 day
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
            VaultRequestUnlock {
                vault: vault.clone(),
                shares: STARTING_SHARES,
            },
        ],
        &[Coin::new(200u128, "uatom"), Coin::new(400u128, "uosmo")],
    )
    .unwrap();

    // Assert token's position with Rover
    let res = mock.query_position(&token_id);
    assert_eq!(res.vault_positions.len(), 1);
    let position = res
        .vault_positions
        .first()
        .unwrap()
        .position
        .unlocking
        .first()
        .unwrap();
    assert_eq!(position.amount, STARTING_SHARES);
    let expected_unlock_time = mock.app.block_info().time.seconds()
        + leverage_vault.lockup.unwrap()
        + leverage_vault.unlock_request_queue.unwrap();
    let unlocking_position = mock.query_unlocking_position_info(&vault, position.id);
    assert_eq!(
        unlocking_position.unlocked_at.seconds(),
        expected_unlock_time
    );

    // Assert Rover's position w/ Vault
    let res = mock.query_unlocking_positions(&vault, &mock.rover);
    assert_eq!(res.len(), 1);
    assert_eq!(res.first().unwrap().amount, STARTING_SHARES);
    assert_eq!(
        res.first().unwrap().unlocked_at.seconds(),
        expected_unlock_time
    );
}

// test only owner can request unlock
// test trying to request when unnecessary
// test requested too many shares to unlock
// test unlock time is the same as the vault reqs
// must emulate the passage of time
// same batch vs separate
