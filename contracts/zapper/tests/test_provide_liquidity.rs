use cosmwasm_std::{coin, Uint128};
use cw_dex::CwDexError;
use osmosis_testing::{Account, Bank, Gamm, Module, OsmosisTestApp, Wasm};

use mars_zapper::msg::{ExecuteMsg, QueryMsg};

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

    let res_err = wasm
        .execute(
            &contract_addr,
            &ExecuteMsg::ProvideLiquidity {
                lp_token_out: pool_denom.clone(),
                recipient: None,
                minimum_receive: estimate_amount,
            },
            &coins_in,
            user,
        )
        .unwrap_err();
    // FIXME: not equal because of fees?
    assert_err(
        res_err,
        CwDexError::MinOutNotReceived {
            min_out: estimate_amount,
            received: Uint128::from(24999975000000000000u128),
        },
    );

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 0u128);
}

#[test]
fn test_provide_liquidity_successfully() {
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

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 0u128);

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::ProvideLiquidity {
            lp_token_out: pool_denom.clone(),
            recipient: None,
            minimum_receive: Uint128::one(),
        },
        &[coin(5_000_000, "uatom"), coin(10_000_000, "uosmo")],
        user,
    )
    .unwrap();

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 24999975000000000000u128);
}

#[test]
fn test_provide_liquidity_with_different_recipient_successfully() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let accs = app
        .init_accounts(
            &[
                coin(1_000_000_000_000, "uatom"),
                coin(1_000_000_000_000, "uosmo"),
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

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::ProvideLiquidity {
            lp_token_out: pool_denom.clone(),
            recipient: Some(recipient.address()),
            minimum_receive: Uint128::one(),
        },
        &[coin(5_000_000, "uatom"), coin(10_000_000, "uosmo")],
        user,
    )
    .unwrap();

    let contract_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_balance, 0u128);
    let user_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_balance, 0u128);
    let recipient_balance = query_balance(&bank, &recipient.address(), &pool_denom);
    assert_eq!(recipient_balance, 24999975000000000000u128);
}
