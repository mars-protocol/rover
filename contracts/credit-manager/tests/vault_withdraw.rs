use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_multi_test::{App, Executor};

use mock_vault::contract::STARTING_SHARES;
use rover::msg::execute::Action::{Deposit, VaultDeposit, VaultWithdraw};
use rover::msg::ExecuteMsg;

use crate::helpers::{
    deploy_vault, fund_vault, get_asset, get_token_id, mock_create_credit_account, query_balance,
    query_position, setup_credit_manager, setup_oracle, CoinInfo, VaultInfo,
};

pub mod helpers;

#[test]
fn test_withdraw_with_unlocked_vault() {
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
        lockup: None,
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
                VaultWithdraw {
                    vault: vault.clone().into(),
                    shares: STARTING_SHARES,
                },
            ],
        },
        &[Coin::new(200u128, "uatom"), Coin::new(400u128, "uosmo")],
    )
    .unwrap();

    // Assert token's updated position
    let res = query_position(&app, &mock.credit_manager, &token_id);
    // assert_eq!(res.vault_positions.len(), 0); TODO: Add purge callback
    let atom = get_asset("uatom", &res.coin_assets);
    assert_eq!(atom.amount, Uint128::from(200u128));
    let osmo = get_asset("uosmo", &res.coin_assets);
    assert_eq!(osmo.amount, Uint128::from(400u128));

    // Assert Rover indeed has those on hand in the bank
    let atom = query_balance(&mut app, &mock.credit_manager, "uatom");
    assert_eq!(atom.amount, Uint128::from(200u128));
    let osmo = query_balance(&mut app, &mock.credit_manager, "uosmo");
    assert_eq!(osmo.amount, Uint128::from(400u128));

    // Assert Rover does not have the LP tokens anymore
    let lp_balance = query_balance(
        &mut app,
        &mock.credit_manager,
        &leverage_vault.lp_token_denom,
    );
    assert_eq!(Uint128::zero(), lp_balance.amount);
}

// test withdraw works on uneven numbers. Is vault supposed to round down? e.g. 23 / 2
// Separate actions (two transactions), versus one big one (deposit + withdraw in same)
// test_withdraw_with_unlocked_vault
// test only token_owner can do a vault withdraw
// test_failure_when_withdrawing_when_no_unlocked_positions_available
// assert vault is whitelisted
// test if no unlocked but only unlocking ones that are ready to be withdrawn
