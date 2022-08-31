use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use rover::coins::Coins;
use rover::error::ContractError::{
    ExtraFundsReceived, FundsMismatch, NotTokenOwner, NotWhitelisted,
};
use rover::msg::execute::Action;
use rover::msg::query::PositionsWithValueResponse;

use crate::helpers::{
    assert_err, uatom_info, ujake_info, uosmo_info, AccountToFund, CoinInfo, MockEnv,
};

pub mod helpers;

#[test]
fn test_only_owner_of_token_can_deposit() {
    let mut mock = MockEnv::new().build().unwrap();
    let user = Addr::unchecked("user");
    let token_id = mock.create_credit_account(&user).unwrap();

    let another_user = Addr::unchecked("another_user");
    let res = mock.update_credit_account(
        &token_id,
        &another_user,
        vec![Action::Deposit(Coin {
            denom: "uosmo".to_string(),
            amount: Uint128::zero(),
        })],
        &[],
    );

    assert_err(
        res,
        NotTokenOwner {
            user: another_user.into(),
            token_id,
        },
    )
}

#[test]
fn test_deposit_nothing() {
    let coin_info = uosmo_info();

    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .build()
        .unwrap();
    let user = Addr::unchecked("user");
    let token_id = mock.create_credit_account(&user).unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Deposit(coin_info.to_coin(Uint128::zero()))],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_deposit_but_no_funds() {
    let coin_info = uosmo_info();

    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .build()
        .unwrap();
    let user = Addr::unchecked("user");
    let token_id = mock.create_credit_account(&user).unwrap();

    let deposit_amount = Uint128::new(234);
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Deposit(coin_info.to_coin(deposit_amount))],
        &[],
    );

    assert_err(
        res,
        FundsMismatch {
            expected: deposit_amount,
            received: Uint128::zero(),
        },
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_deposit_but_not_enough_funds() {
    let coin_info = uosmo_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![Coin::new(300u128, coin_info.denom.clone())],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Deposit(coin_info.to_coin(Uint128::new(350)))],
        &[Coin::new(250u128, coin_info.denom)],
    );

    assert_err(
        res,
        FundsMismatch {
            expected: Uint128::new(350),
            received: Uint128::new(250),
        },
    );
}

#[test]
fn test_can_only_deposit_allowed_assets() {
    let coin_info = uosmo_info();
    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![Coin::new(300u128, coin_info.denom.clone())],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let not_allowed_coin = ujake_info().to_coin(Uint128::new(234));

    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Deposit(not_allowed_coin.clone())],
        &[Coin::new(250u128, coin_info.denom)],
    );

    assert_err(res, NotWhitelisted(not_allowed_coin.denom));

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_extra_funds_received() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![
                Coin::new(300u128, uosmo_info.denom.clone()),
                Coin::new(250u128, uatom_info.denom.clone()),
            ],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let extra_funds = Coin::new(25u128, uatom_info.denom);
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Deposit(uosmo_info.to_coin(Uint128::new(234)))],
        &[Coin::new(234u128, uosmo_info.denom), extra_funds.clone()],
    );

    assert_err(res, ExtraFundsReceived(Coins::from(vec![extra_funds])));

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_deposit_success() {
    let coin_info = uosmo_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![Coin::new(300u128, coin_info.denom.clone())],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let deposit_amount = Uint128::new(234);
    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Deposit(coin_info.to_coin(deposit_amount))],
        &[Coin::new(deposit_amount.into(), coin_info.denom.clone())],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    let assets_res = res.coins.first().unwrap();
    assert_eq!(res.coins.len(), 1);
    assert_eq!(assets_res.amount, deposit_amount);
    assert_eq!(assets_res.denom, coin_info.denom);
    assert_eq!(assets_res.price, coin_info.price);
    assert_eq!(
        assets_res.value,
        coin_info.price * Decimal::from_atomics(deposit_amount, 0).unwrap()
    );

    let coin = mock.query_balance(&mock.rover, &coin_info.denom);
    assert_eq!(coin.amount, deposit_amount)
}

#[test]
fn test_multiple_deposit_actions() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![
                Coin::new(300u128, uosmo_info.denom.clone()),
                Coin::new(50u128, uatom_info.denom.clone()),
            ],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let uosmo_amount = Uint128::new(234);
    let uatom_amount = Uint128::new(25);

    mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(uosmo_info.to_coin(uosmo_amount)),
            Action::Deposit(uatom_info.to_coin(uatom_amount)),
        ],
        &[
            Coin::new(234u128, uosmo_info.denom.clone()),
            Coin::new(25u128, uatom_info.denom.clone()),
        ],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 2);
    let uosmo_value = Decimal::from_atomics(uosmo_amount, 0).unwrap() * uosmo_info.price;
    assert_present(&res, &uosmo_info, uosmo_amount, uosmo_value);
    let uatom_value = Decimal::from_atomics(uatom_amount, 0).unwrap() * uatom_info.price;
    assert_present(&res, &uatom_info, uatom_amount, uatom_value);

    let coin = mock.query_balance(&mock.rover, &uosmo_info.denom);
    assert_eq!(coin.amount, uosmo_amount);

    let coin = mock.query_balance(&mock.rover, &uatom_info.denom);
    assert_eq!(coin.amount, uatom_amount);
}

fn assert_present(
    res: &PositionsWithValueResponse,
    coin: &CoinInfo,
    amount: Uint128,
    total_val: Decimal,
) {
    res.coins
        .iter()
        .find(|item| item.denom == coin.denom && item.amount == amount && item.value == total_val)
        .unwrap();
}