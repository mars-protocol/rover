use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_multi_test::{App, Executor};
use mock_vault::contract::STARTING_SHARES;

use rover::msg::execute::Action::{Deposit, VaultDeposit};
use rover::msg::vault::QueryMsg;
use rover::msg::ExecuteMsg;

use crate::helpers::{
    deploy_vault, fund_vault, get_token_id, mock_create_credit_account, query_balance,
    query_position, setup_credit_manager, setup_oracle, CoinInfo, VaultInfo,
};

pub mod helpers;

#[test]
fn test_deposit_into_unlocked_vault() {
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
            ],
        },
        &[Coin::new(200u128, "uatom"), Coin::new(400u128, "uosmo")],
    )
    .unwrap();

    let lp_balance = query_balance(
        &mut app,
        &mock.credit_manager,
        &leverage_vault.lp_token_denom,
    );
    assert_eq!(STARTING_SHARES, lp_balance.amount);

    let res = query_position(&app, &mock.credit_manager, &token_id);
    assert_eq!(res.vault_positions.len(), 1);
    assert_eq!(
        STARTING_SHARES,
        res.vault_positions.first().unwrap().position.unlocked
    );
    assert_eq!(
        Uint128::zero(),
        res.vault_positions.first().unwrap().position.locked
    );

    let assets = query_preview_redeem(
        &app,
        &vault.0,
        res.vault_positions.first().unwrap().position.unlocked,
    );

    let osmo_withdraw = assets.iter().find(|coin| coin.denom == "uosmo").unwrap();
    assert_eq!(osmo_withdraw.amount, Uint128::from(120u128));
    let atom_withdraw = assets.iter().find(|coin| coin.denom == "uatom").unwrap();
    assert_eq!(atom_withdraw.amount, Uint128::from(23u128));
}

pub fn query_preview_redeem(app: &App, vault_contract: &Addr, shares: Uint128) -> Vec<Coin> {
    app.wrap()
        .query_wasm_smart(vault_contract.clone(), &QueryMsg::PreviewRedeem { shares })
        .unwrap()
}

// Assert total shares have been incremented correctly
// test LP tokens are sent back to Rover
// test only token_owner can do a vault deposit
// VaultRequirements query is accurate
// test_deposit_into_unlocked_vault
// adding first transaction to vault
// test multiple vaults to make sure deposits don't leak
// test trying to deposit too much (that they don't have in their wallet)
// test refund is happening if uneven deposit
// in query config, and update config
// assert vault is whitelisted
