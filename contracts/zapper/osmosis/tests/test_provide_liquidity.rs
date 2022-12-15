use cosmwasm_std::{coin, Uint128};
use cw_dex::CwDexError;
use osmosis_testing::{Account, Bank, Gamm, Module, OsmosisTestApp, Wasm};

use mars_zapper_base::{ExecuteMsg, QueryMsg};

use crate::helpers::{assert_err, instantiate_contract, query_balance};

pub mod helpers;

#[test]
fn test_provide_liquidity_with_invalid_lp_token() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app
        .init_account(&[
            coin(1_000_000_000_000, "uatom"),
            coin(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let contract_addr = instantiate_contract(&wasm, &signer);

    let res_err = wasm
        .execute(
            &contract_addr,
            &ExecuteMsg::ProvideLiquidity {
                lp_token_out: "INVALID_POOL".to_string(),
                recipient: None,
                minimum_receive: Uint128::one(),
            },
            &[coin(1_000_000, "uatom"), coin(2_000_000, "uosmo")],
            &signer,
        )
        .unwrap_err();
    assert_err(res_err, CwDexError::NotLpToken {});
}

#[test]
fn test_provide_liquidity_with_invalid_coins() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app
        .init_account(&[
            coin(1_000_000_000_000, "uatom"),
            coin(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let gamm = Gamm::new(&app);
    let pool_id = gamm
        .create_basic_pool(
            &[coin(2_000_000, "uatom"), coin(4_000_000, "uosmo")],
            &signer,
        )
        .unwrap()
        .data
        .pool_id;

    let contract_addr = instantiate_contract(&wasm, &signer);

    // Generic error: Querier contract error: codespace: undefined, code: 1: execute wasm contract failed
    wasm.execute(
        &contract_addr,
        &ExecuteMsg::ProvideLiquidity {
            lp_token_out: format!("gamm/pool/{}", pool_id),
            recipient: None,
            minimum_receive: Uint128::one(),
        },
        &[],
        &signer,
    )
    .unwrap_err();
}

#[test]
fn test_provide_liquidity_with_min_not_received() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let accs = app
        .init_accounts(
            &[
                coin(1_000_000_000_000, "uatom"),
                coin(1_000_000_000_000, "uosmo"),
            ],
            2,
        )
        .unwrap();
    let admin = &accs[0];
    let user = &accs[1];

    let gamm = Gamm::new(&app);
    let pool_id = gamm
        .create_basic_pool(
            &[coin(20_000_000, "uatom"), coin(40_000_000, "uosmo")],
            admin,
        )
        .unwrap()
        .data
        .pool_id;
    let pool_denom = format!("gamm/pool/{}", pool_id);

    let contract_addr = instantiate_contract(&wasm, admin);

    let bank = Bank::new(&app);

    let coins_in = vec![coin(5_000_000, "uatom"), coin(10_000_000, "uosmo")];

    let estimate_amount: Uint128 = wasm
        .query(
            &contract_addr,
            &QueryMsg::EstimateProvideLiquidity {
                lp_token_out: pool_denom.clone(),
                coins_in: coins_in.clone(),
            },
        )
        .unwrap();

    let min_receive = estimate_amount + Uint128::one();
    let res_err = wasm
        .execute(
            &contract_addr,
            &ExecuteMsg::ProvideLiquidity {
                lp_token_out: pool_denom.clone(),
                recipient: None,
                minimum_receive: min_receive,
            },
            &coins_in,
            user,
        )
        .unwrap_err();
    assert_err(
        res_err,
        CwDexError::MinOutNotReceived {
            min_out: min_receive,
            received: Uint128::from(25000000000000000000u128),
        },
    );

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 0u128);
}

#[test]
fn test_provide_liquidity_with_one_coin_successfully() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let uatom_acc_balance = 1_000_000_000_000u128;
    let uosmo_acc_balance = 1_000_000_000_000u128;
    let accs = app
        .init_accounts(
            &[
                coin(uatom_acc_balance, "uatom"),
                coin(uosmo_acc_balance, "uosmo"),
            ],
            2,
        )
        .unwrap();
    let admin = &accs[0];
    let user = &accs[1];

    let gamm = Gamm::new(&app);
    let pool_id = gamm
        .create_basic_pool(
            &[coin(20_000_000, "uatom"), coin(40_000_000, "uosmo")],
            admin,
        )
        .unwrap()
        .data
        .pool_id;
    let pool_denom = format!("gamm/pool/{}", pool_id);

    let contract_addr = instantiate_contract(&wasm, admin);

    let bank = Bank::new(&app);

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 0u128);

    let uatom_liquidity_amount = 1_000_000u128;
    let coins_in = vec![coin(uatom_liquidity_amount, "uatom")];

    let estimate_amount: Uint128 = wasm
        .query(
            &contract_addr,
            &QueryMsg::EstimateProvideLiquidity {
                lp_token_out: pool_denom.clone(),
                coins_in: coins_in.clone(),
            },
        )
        .unwrap();
    assert_eq!(estimate_amount.u128(), 2457308182481546200u128);

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::ProvideLiquidity {
            lp_token_out: pool_denom.clone(),
            recipient: None,
            minimum_receive: estimate_amount,
        },
        &coins_in,
        user,
    )
    .unwrap();

    let contract_pool_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_pool_balance, 0u128);
    let contract_uatom_balance = query_balance(&bank, &contract_addr, "uatom");
    assert_eq!(contract_uatom_balance, 0u128);
    let contract_uosmo_balance = query_balance(&bank, &contract_addr, "uosmo");
    assert_eq!(contract_uosmo_balance, 0u128);

    let user_pool_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_pool_balance, estimate_amount.u128());
    let user_uatom_balance = query_balance(&bank, &user.address(), "uatom");
    assert_eq!(
        user_uatom_balance,
        uatom_acc_balance - uatom_liquidity_amount
    );
    let user_uosmo_balance = query_balance(&bank, &user.address(), "uosmo");
    assert_eq!(user_uosmo_balance, uosmo_acc_balance);
}

#[test]
fn test_provide_liquidity_with_two_coins_successfully() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let uatom_acc_balance = 1_000_000_000_000u128;
    let uosmo_acc_balance = 1_000_000_000_000u128;
    let accs = app
        .init_accounts(
            &[
                coin(uatom_acc_balance, "uatom"),
                coin(uosmo_acc_balance, "uosmo"),
            ],
            2,
        )
        .unwrap();
    let admin = &accs[0];
    let user = &accs[1];

    let gamm = Gamm::new(&app);
    let pool_id = gamm
        .create_basic_pool(
            &[coin(20_000_000, "uatom"), coin(40_000_000, "uosmo")],
            admin,
        )
        .unwrap()
        .data
        .pool_id;
    let pool_denom = format!("gamm/pool/{}", pool_id);

    let contract_addr = instantiate_contract(&wasm, admin);

    let bank = Bank::new(&app);

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 0u128);

    let uatom_liquidity_amount = 5_000_000u128;
    let uosmo_liquidity_amount = 10_000_000u128;
    let coins_in = vec![
        coin(uatom_liquidity_amount, "uatom"),
        coin(uosmo_liquidity_amount, "uosmo"),
    ];

    let estimate_amount: Uint128 = wasm
        .query(
            &contract_addr,
            &QueryMsg::EstimateProvideLiquidity {
                lp_token_out: pool_denom.clone(),
                coins_in: coins_in.clone(),
            },
        )
        .unwrap();
    assert_eq!(estimate_amount.u128(), 25000000000000000000u128);

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::ProvideLiquidity {
            lp_token_out: pool_denom.clone(),
            recipient: None,
            minimum_receive: estimate_amount,
        },
        &coins_in,
        user,
    )
    .unwrap();

    let contract_pool_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_pool_balance, 0u128);
    let contract_uatom_balance = query_balance(&bank, &contract_addr, "uatom");
    assert_eq!(contract_uatom_balance, 0u128);
    let contract_uosmo_balance = query_balance(&bank, &contract_addr, "uosmo");
    assert_eq!(contract_uosmo_balance, 0u128);

    let user_pool_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_pool_balance, estimate_amount.u128());
    let user_uatom_balance = query_balance(&bank, &user.address(), "uatom");
    assert_eq!(
        user_uatom_balance,
        uatom_acc_balance - uatom_liquidity_amount
    );
    let user_uosmo_balance = query_balance(&bank, &user.address(), "uosmo");
    assert_eq!(
        user_uosmo_balance,
        uosmo_acc_balance - uosmo_liquidity_amount
    );
}

#[test]
fn test_provide_liquidity_with_different_recipient_successfully() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let uatom_acc_balance = 1_000_000_000_000u128;
    let uosmo_acc_balance = 1_000_000_000_000u128;
    let accs = app
        .init_accounts(
            &[
                coin(uatom_acc_balance, "uatom"),
                coin(uosmo_acc_balance, "uosmo"),
            ],
            3,
        )
        .unwrap();
    let admin = &accs[0];
    let user = &accs[1];
    let recipient = &accs[2];

    let gamm = Gamm::new(&app);
    let pool_id = gamm
        .create_basic_pool(
            &[coin(20_000_000, "uatom"), coin(40_000_000, "uosmo")],
            admin,
        )
        .unwrap()
        .data
        .pool_id;
    let pool_denom = format!("gamm/pool/{}", pool_id);

    let contract_addr = instantiate_contract(&wasm, admin);

    let bank = Bank::new(&app);

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 0u128);
    let recipient_balance = query_balance(&bank, &recipient.address(), &pool_denom);
    assert_eq!(recipient_balance, 0u128);

    let uatom_liquidity_amount = 5_000_000u128;
    let uosmo_liquidity_amount = 10_000_000u128;
    let coins_in = vec![
        coin(uatom_liquidity_amount, "uatom"),
        coin(uosmo_liquidity_amount, "uosmo"),
    ];

    let estimate_amount: Uint128 = wasm
        .query(
            &contract_addr,
            &QueryMsg::EstimateProvideLiquidity {
                lp_token_out: pool_denom.clone(),
                coins_in: coins_in.clone(),
            },
        )
        .unwrap();
    assert_eq!(estimate_amount.u128(), 25000000000000000000u128);

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::ProvideLiquidity {
            lp_token_out: pool_denom.clone(),
            recipient: Some(recipient.address()),
            minimum_receive: estimate_amount,
        },
        &coins_in,
        user,
    )
    .unwrap();

    let contract_pool_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_pool_balance, 0u128);
    let contract_uatom_balance = query_balance(&bank, &contract_addr, "uatom");
    assert_eq!(contract_uatom_balance, 0u128);
    let contract_uosmo_balance = query_balance(&bank, &contract_addr, "uosmo");
    assert_eq!(contract_uosmo_balance, 0u128);

    let user_pool_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_pool_balance, 0u128);
    let user_uatom_balance = query_balance(&bank, &user.address(), "uatom");
    assert_eq!(
        user_uatom_balance,
        uatom_acc_balance - uatom_liquidity_amount
    );
    let user_uosmo_balance = query_balance(&bank, &user.address(), "uosmo");
    assert_eq!(
        user_uosmo_balance,
        uosmo_acc_balance - uosmo_liquidity_amount
    );

    let recipient_pool_balance = query_balance(&bank, &recipient.address(), &pool_denom);
    assert_eq!(recipient_pool_balance, estimate_amount.u128());
    let recipient_uatom_balance = query_balance(&bank, &recipient.address(), "uatom");
    assert_eq!(recipient_uatom_balance, uatom_acc_balance);
    let recipient_uosmo_balance = query_balance(&bank, &recipient.address(), "uosmo");
    assert_eq!(recipient_uosmo_balance, uosmo_acc_balance);
}