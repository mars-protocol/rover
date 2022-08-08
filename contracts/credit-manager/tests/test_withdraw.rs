use cosmwasm_std::OverflowOperation::Sub;
use cosmwasm_std::{Addr, Coin, Decimal, OverflowError, Uint128};

use rover::error::ContractError;
use rover::error::ContractError::{NotTokenOwner, NotWhitelisted};
use rover::msg::execute::Action;

use crate::helpers::{assert_err, AccountToFund, CoinInfo, MockEnv};

pub mod helpers;

#[test]
fn test_only_owner_of_token_can_withdraw() {
    let coin_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
    let owner = Addr::unchecked("owner");
    let mut mock = MockEnv::new().build().unwrap();
    let token_id = mock.create_credit_account(&owner).unwrap();

    let another_user = Addr::unchecked("another_user");
    let res = mock.update_credit_account(
        &token_id,
        &another_user,
        vec![Action::Withdraw(Coin {
            denom: coin_info.denom,
            amount: Uint128::new(382),
        })],
        &[],
    );

    assert_err(
        res,
        NotTokenOwner {
            user: another_user.into(),
            token_id: token_id.clone(),
        },
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_nothing() {
    let coin_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(Coin {
            denom: coin_info.denom,
            amount: Uint128::new(0),
        })],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_but_no_funds() {
    let coin_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[coin_info.clone()])
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let withdraw_amount = Uint128::from(234u128);
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(coin_info.to_coin(withdraw_amount))],
        &[],
    );

    assert_err(
        res,
        ContractError::Overflow(OverflowError {
            operation: Sub,
            operand1: "0".to_string(),
            operand2: "234".to_string(),
        }),
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_but_not_enough_funds() {
    let coin_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
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
        vec![
            Action::Deposit(coin_info.to_coin(Uint128::from(300u128))),
            Action::Withdraw(coin_info.to_coin(Uint128::from(400u128))),
        ],
        &[Coin::new(300u128, coin_info.denom)],
    );

    assert_err(
        res,
        ContractError::Overflow(OverflowError {
            operation: Sub,
            operand1: "300".to_string(),
            operand2: "400".to_string(),
        }),
    );

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_can_only_withdraw_allowed_assets() {
    let coin_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
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

    let not_allowed_coin = Coin {
        denom: "ujakecoin".to_string(),
        amount: Uint128::from(234u128),
    };
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(coin_info.to_coin(Uint128::from(234u128))),
            Action::Withdraw(not_allowed_coin.clone()),
        ],
        &[Coin::new(234u128, coin_info.denom)],
    );

    assert_err(res, NotWhitelisted(not_allowed_coin.denom));

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_cannot_withdraw_more_than_healthy() {
    let coin_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
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

    let deposit_amount = Uint128::from(200u128);
    let res = mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(coin_info.to_coin(deposit_amount)),
            Action::Borrow(coin_info.to_coin(Uint128::from(400u128))),
            Action::Withdraw(coin_info.to_coin(Uint128::from(50u128))),
        ],
        &[Coin::new(200u128, coin_info.denom)],
    );

    assert_err(res, ContractError::AboveMaxLTV);

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);
}

#[test]
fn test_withdraw_success() {
    let coin_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
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

    let deposit_amount = Uint128::from(234u128);
    mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Action::Deposit(coin_info.to_coin(deposit_amount)),
            Action::Withdraw(coin_info.to_coin(deposit_amount)),
        ],
        &[Coin::new(deposit_amount.into(), coin_info.denom.clone())],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);

    let coin = mock.query_balance(&mock.rover, &coin_info.denom);
    assert_eq!(coin.amount, Uint128::zero())
}

#[test]
fn test_multiple_withdraw_actions() {
    let uosmo_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };
    let uatom_info = CoinInfo {
        denom: "uatom".to_string(),
        price: Decimal::from_atomics(10u128, 1).unwrap(),
        max_ltv: Decimal::from_atomics(82u128, 2).unwrap(),
        liquidation_threshold: Decimal::from_atomics(9u128, 1).unwrap(),
    };

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![
                Coin::new(234u128, uosmo_info.denom.clone()),
                Coin::new(25u128, uatom_info.denom.clone()),
            ],
        })
        .build()
        .unwrap();
    let token_id = mock.create_credit_account(&user).unwrap();

    let uosmo_amount = Uint128::from(234u128);
    let uatom_amount = Uint128::from(25u128);

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

    let coin = mock.query_balance(&user, &uosmo_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    let coin = mock.query_balance(&user, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(uosmo_info.to_coin(uosmo_amount))],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 1);

    let coin = mock.query_balance(&mock.rover, &uosmo_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    let coin = mock.query_balance(&user, &uosmo_info.denom);
    assert_eq!(coin.amount, uosmo_amount);

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(uatom_info.to_coin(Uint128::from(20u128)))],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 1);

    let coin = mock.query_balance(&mock.rover, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::from(5u128));

    let coin = mock.query_balance(&user, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::from(20u128));

    mock.update_credit_account(
        &token_id,
        &user,
        vec![Action::Withdraw(uatom_info.to_coin(Uint128::from(5u128)))],
        &[],
    )
    .unwrap();

    let res = mock.query_position(&token_id);
    assert_eq!(res.coins.len(), 0);

    let coin = mock.query_balance(&mock.rover, &uatom_info.denom);
    assert_eq!(coin.amount, Uint128::zero());

    let coin = mock.query_balance(&user, &uatom_info.denom);
    assert_eq!(coin.amount, uatom_amount);
}
