use cosmwasm_std::{coin, Uint128};
use cw_dex::CwDexError;
use osmosis_testing::{Account, Bank, Gamm, Module, OsmosisTestApp, Wasm};

use mars_zapper::msg::ExecuteMsg;

use crate::helpers::{assert_err, instantiate_contract, query_balance};

pub mod helpers;

#[test]
fn test_withdraw_liquidity_with_more_than_one_coin_sent() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app
        .init_account(&[
            coin(1_000_000_000_000, "gamm/pool/1"),
            coin(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let contract_addr = instantiate_contract(&wasm, &signer);

    let res_err = wasm
        .execute(
            &contract_addr,
            &ExecuteMsg::WithdrawLiquidity { recipient: None },
            &[coin(1_000_000, "gamm/pool/1"), coin(2_000_000, "uosmo")],
            &signer,
        )
        .unwrap_err();
    assert_err(res_err, "More than one coin sent to Withdraw Liquidity");
}

#[test]
fn test_withdraw_liquidity_with_invalid_lp_token() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app
        .init_account(&[coin(1_000_000_000_000, "uosmo")])
        .unwrap();

    let contract_addr = instantiate_contract(&wasm, &signer);

    let res_err = wasm
        .execute(
            &contract_addr,
            &ExecuteMsg::WithdrawLiquidity { recipient: None },
            &[coin(1_000_000, "uosmo")],
            &signer,
        )
        .unwrap_err();
    assert_err(res_err, CwDexError::NotLpToken {});
}

#[test]
fn test_withdraw_liquidity_successfully() {
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

    let contract_pool_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_pool_balance, 0u128);
    let contract_uatom_balance = query_balance(&bank, &contract_addr, "uatom");
    assert_eq!(contract_uatom_balance, 0u128);
    let contract_uosmo_balance = query_balance(&bank, &contract_addr, "uosmo");
    assert_eq!(contract_uosmo_balance, 0u128);

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

    let contract_pool_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_pool_balance, 0u128);
    // FIXME: should be zeros?
    let contract_uatom_balance = query_balance(&bank, &contract_addr, "uatom");
    assert_eq!(contract_uatom_balance, 5u128);
    let contract_uosmo_balance = query_balance(&bank, &contract_addr, "uosmo");
    assert_eq!(contract_uosmo_balance, 10u128);

    let user_pool_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_pool_balance, 24999975000000000000u128);
    let user_uatom_balance = query_balance(&bank, &user.address(), "uatom");
    assert_eq!(user_uatom_balance, 999995000000u128);
    let user_uosmo_balance = query_balance(&bank, &user.address(), "uosmo");
    assert_eq!(user_uosmo_balance, 999990000000u128);

    // FIXME: lack of Vec<Coin> in response
    /*let estimate_coins: Vec<Coin> = wasm
        .query(
            &contract_addr,
            &QueryMsg::EstimateWithdrawLiquidity {
                coin_in: coin(100_000u128, &pool_denom),
            },
        )
        .unwrap();
    println!("coins: {:?}", estimate_coins);
    let uatom_amount = estimate_coins.iter().find(|c| c.denom.as_ref() == "uatom").unwrap().amount;
    let uosmo_amount = estimate_coins.iter().find(|c| c.denom.as_ref() == "uosmo").unwrap().amount;
    assert_eq!(uatom_amount, uosmo_amount);*/

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::WithdrawLiquidity { recipient: None },
        &[coin(user_pool_balance, &pool_denom)],
        user,
    )
    .unwrap();

    let contract_pool_balance = query_balance(&bank, &contract_addr, &pool_denom);
    assert_eq!(contract_pool_balance, 0u128);
    let contract_uatom_balance = query_balance(&bank, &contract_addr, "uatom");
    assert_eq!(contract_uatom_balance, 5u128);
    let contract_uosmo_balance = query_balance(&bank, &contract_addr, "uosmo");
    assert_eq!(contract_uosmo_balance, 10u128);

    let user_pool_balance = query_balance(&bank, &user.address(), &pool_denom);
    assert_eq!(user_pool_balance, 0u128);
    let user_uatom_balance = query_balance(&bank, &user.address(), "uatom");
    assert_eq!(user_uatom_balance, 999999949995u128);
    let user_uosmo_balance = query_balance(&bank, &user.address(), "uosmo");
    assert_eq!(user_uosmo_balance, 999999899990u128);
}

/*#[test]
fn test_withdraw_liquidity_with_different_recipient_successfully() {
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
}*/
