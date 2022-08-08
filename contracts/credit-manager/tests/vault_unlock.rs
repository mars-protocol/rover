use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use mock_vault::contract::STARTING_SHARES;
use rover::msg::execute::Action::{Deposit, VaultDeposit, VaultRequestUnlock, VaultUnlock};
use rover::msg::query::PositionResponse;

use crate::helpers::{get_asset, AccountToFund, CoinInfo, MockEnv, VaultTestInfo};

pub mod helpers;

#[test]
fn test_unlock() {
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
        liquidation_threshold: Decimal::from_atomics(8u128, 1).unwrap(),
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

    mock.app.update_block(|block| {
        block.time = block
            .time
            .plus_seconds(leverage_vault.lockup.unwrap())
            .plus_seconds(leverage_vault.unlock_request_queue.unwrap());
        block.height += 1;
    });

    let PositionResponse {
        vault_positions, ..
    } = mock.query_position(&token_id);

    let position_id = vault_positions
        .first()
        .unwrap()
        .position
        .unlocking
        .first()
        .unwrap()
        .id;

    mock.update_credit_account(
        &token_id,
        &user,
        vec![VaultUnlock {
            id: position_id,
            vault,
        }],
        &[],
    )
    .unwrap();

    let PositionResponse {
        vault_positions,
        coins,
        ..
    } = mock.query_position(&token_id);

    // Users vault position decrements
    let position = vault_positions.first().unwrap().position.clone();
    // assert_eq!(vault_positions.len(), 0); TODO: Add purge callback
    assert_eq!(position.unlocking.len(), 0);
    assert_eq!(position.unlocked, Uint128::zero());
    assert_eq!(position.locked, Uint128::zero());

    // Users asset position decrements
    let atom = get_asset("uatom", &coins);
    assert_eq!(atom.amount, Uint128::from(200u128));
    let osmo = get_asset("uosmo", &coins);
    assert_eq!(osmo.amount, Uint128::from(400u128));

    // Assert Rover indeed has those on hand in the bank
    let atom = mock.query_balance(&mock.rover, "uatom");
    assert_eq!(atom.amount, Uint128::from(200u128));
    let osmo = mock.query_balance(&mock.rover, "uosmo");
    assert_eq!(osmo.amount, Uint128::from(400u128));
}

// assert only whitelisted vaults can have actions taken
