use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_multi_test::{App, Executor};

use mock_vault::contract::STARTING_SHARES;
use rover::adapters::Vault;
use rover::msg::execute::Action::{Deposit, VaultDeposit, VaultRequestUnlock};
use rover::msg::vault::{QueryMsg, UnlockingTokens};
use rover::msg::ExecuteMsg;

use crate::helpers::{
    deploy_vault, fund_vault, get_token_id, mock_create_credit_account, query_position,
    setup_credit_manager, setup_oracle, CoinInfo, VaultInfo,
};

pub mod helpers;

#[test]
fn test_request_unlocked() {
    let user = Addr::unchecked("user");
    let mut app = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &user,
                vec![Coin::new(300u128, "uatom"), Coin::new(500u128, "uosmo")],
            )
            .unwrap();
    });

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

    let leverage_vault = VaultInfo {
        lp_token_denom: "uleverage".to_string(),
        lockup: Some(1_209_600), // 14 days
        asset_denoms: vec!["uatom".to_string(), "uosmo".to_string()],
        pre_fund: Uint128::from(100_000_000u128),
    };

    let oracle = setup_oracle(&mut app, &vec![uatom.clone(), uosmo.clone()]);

    let vault = deploy_vault(&mut app, leverage_vault.clone(), oracle);

    fund_vault(
        &mut app,
        vault.clone(),
        vec![Coin {
            denom: leverage_vault.lp_token_denom.clone(),
            amount: leverage_vault.pre_fund.clone(),
        }],
    );

    let mock = setup_credit_manager(
        &mut app,
        &Addr::unchecked("owner"),
        vec![uatom.clone(), uosmo.clone()],
        vec![vault.clone().into()],
    );

    let res = mock_create_credit_account(&mut app, &mock.credit_manager, &user).unwrap();
    let token_id = get_token_id(res);

    app.execute_contract(
        user.clone(),
        mock.credit_manager.clone(),
        &ExecuteMsg::UpdateCreditAccount {
            token_id: token_id.clone(),
            actions: vec![
                Deposit(Coin {
                    denom: uatom.denom.clone(),
                    amount: Uint128::from(200u128),
                }),
                Deposit(Coin {
                    denom: uosmo.denom.clone(),
                    amount: Uint128::from(400u128),
                }),
                VaultDeposit {
                    vault: vault.clone().into(),
                    assets: vec![Coin::new(23u128, "uatom"), Coin::new(120u128, "uosmo")],
                },
                VaultRequestUnlock {
                    vault: vault.clone().into(),
                    shares: STARTING_SHARES,
                },
            ],
        },
        &[Coin::new(200u128, "uatom"), Coin::new(400u128, "uosmo")],
    )
    .unwrap();

    // Assert token's position with Rover
    let res = query_position(&app, &mock.credit_manager, &token_id);
    assert_eq!(res.vault_positions.len(), 1);
    let position = res
        .vault_positions
        .first()
        .unwrap()
        .position
        .unlocking
        .first()
        .unwrap();
    let expected_unlock_time = app.block_info().time.seconds() + leverage_vault.lockup.unwrap();
    assert_eq!(position.amount, STARTING_SHARES);
    assert_eq!(position.unlocked_at.seconds(), expected_unlock_time);

    // Assert Rover's position w/ Vault
    let res = query_unlocking_positions(&app, vault, &mock.credit_manager);
    assert_eq!(res.len(), 1);
    assert_eq!(res.first().unwrap().amount, STARTING_SHARES);
    assert_eq!(
        res.first().unwrap().unlocked_at.seconds(),
        expected_unlock_time
    );
}

fn query_unlocking_positions(
    app: &App,
    vault: Vault,
    manager_contract_addr: &Addr,
) -> Vec<UnlockingTokens> {
    app.wrap()
        .query_wasm_smart(
            vault.0.to_string(),
            &QueryMsg::Unlocking {
                addr: manager_contract_addr.to_string(),
            },
        )
        .unwrap()
}

// test only owner can request unlock
// test trying to request when unnecessary
// test requested too many shares to unlock
// test unlock time is the same as the vault reqs
// must emulate the passage of time
// same batch vs separate
