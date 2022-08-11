use std::ops::Sub;

use cosmwasm_std::{Addr, Coin, Decimal, OverflowError, OverflowOperation, Uint128};

use mock_oracle::msg::CoinPrice;
use rover::error::ContractError;
use rover::error::ContractError::{AboveMaxLTV, NotLiquidatable};
use rover::msg::execute::Action::{Borrow, Deposit, LiquidateCoin};
use rover::msg::query::{CoinValue, DebtSharesValue};

use crate::helpers::{assert_err, uatom_info, uosmo_info, AccountToFund, CoinInfo, MockEnv};

pub mod helpers;

#[test]
fn test_can_only_liquidate_unhealthy_accounts() {
    let uosmo_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(23654u128, 4).unwrap(),
        max_ltv: Decimal::from_atomics(5u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(55u128, 2).unwrap(),
    };
    let uatom_info = CoinInfo {
        denom: "uatom".to_string(),
        price: Decimal::from_atomics(102u128, 1).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(75u128, 2).unwrap(),
    };

    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(50u128))),
        ],
        &[Coin::new(300, uosmo_info.clone().denom)],
    )
    .unwrap();

    let health = mock.query_health(&liquidatee_token_id);
    assert!(!health.liquidatable);

    let liquidator = Addr::unchecked("liquidator");
    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![LiquidateCoin {
            liquidatee_token_id: liquidatee_token_id.clone(),
            debt: uatom_info.to_coin(Uint128::from(10u128)),
            request_coin: uosmo_info.to_coin(Uint128::from(50u128)),
        }],
        &[],
    );

    assert_err(
        res,
        NotLiquidatable {
            token_id: liquidatee_token_id,
            lqdt_health_factor: "1.485565167243367935".to_string(),
        },
    )
}

#[test]
fn test_liquidatee_does_not_have_requested_asset() {
    let uosmo_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(23654u128, 4).unwrap(),
        max_ltv: Decimal::from_atomics(5u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(55u128, 2).unwrap(),
    };
    let uatom_info = CoinInfo {
        denom: "uatom".to_string(),
        price: Decimal::from_atomics(102u128, 1).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(75u128, 2).unwrap(),
    };
    let ujake_info = CoinInfo {
        denom: "ujake".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };

    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone(), ujake_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(105u128))),
        ],
        &[Coin::new(300, uosmo_info.denom)],
    )
    .unwrap();

    let health = mock.query_health(&liquidatee_token_id);
    assert!(!health.liquidatable);

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let liquidator = Addr::unchecked("liquidator");
    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![
            Borrow(uatom_info.to_coin(Uint128::from(50u128))),
            LiquidateCoin {
                liquidatee_token_id: liquidatee_token_id.clone(),
                debt: uatom_info.to_coin(Uint128::from(10u128)),
                request_coin: ujake_info.to_coin(Uint128::from(50u128)),
            },
        ],
        &[],
    );

    assert_err(res, ContractError::CoinNotAvailable(ujake_info.denom))
}

#[test]
fn test_liquidatee_does_not_have_debt_coin() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();
    let ujake_info = CoinInfo {
        denom: "ujake".to_string(),
        price: Decimal::from_atomics(25u128, 2).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(78u128, 2).unwrap(),
    };

    let liquidatee = Addr::unchecked("liquidatee");
    let random_user = Addr::unchecked("random_user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone(), ujake_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .fund_account(AccountToFund {
            addr: random_user.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(105u128))),
        ],
        &[Coin::new(300, uosmo_info.denom.clone())],
    )
    .unwrap();

    let health = mock.query_health(&liquidatee_token_id);
    assert!(!health.liquidatable);

    // Seeding a jakecoin borrow
    let random_user_token = mock.create_credit_account(&random_user).unwrap();
    mock.update_credit_account(
        &random_user_token,
        &random_user,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(ujake_info.to_coin(Uint128::from(10u128))),
        ],
        &[Coin::new(300, uosmo_info.denom)],
    )
    .unwrap();

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let liquidator = Addr::unchecked("liquidator");
    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![
            Borrow(uatom_info.to_coin(Uint128::from(50u128))),
            LiquidateCoin {
                liquidatee_token_id: liquidatee_token_id.clone(),
                debt: ujake_info.to_coin(Uint128::from(10u128)),
                request_coin: uatom_info.to_coin(Uint128::from(50u128)),
            },
        ],
        &[],
    );

    assert_err(res, ContractError::NoDebt)
}

#[test]
fn test_liquidator_does_not_have_enough_to_pay_debt() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();

    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(100u128))),
        ],
        &[Coin::new(300, uosmo_info.clone().denom)],
    )
    .unwrap();

    let health = mock.query_health(&liquidatee_token_id);
    assert!(!health.liquidatable);

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let liquidator = Addr::unchecked("liquidator");
    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![LiquidateCoin {
            liquidatee_token_id: liquidatee_token_id.clone(),
            debt: uatom_info.to_coin(Uint128::from(10u128)),
            request_coin: uosmo_info.to_coin(Uint128::from(50u128)),
        }],
        &[],
    );

    assert_err(
        res,
        ContractError::Overflow(OverflowError {
            operation: OverflowOperation::Sub,
            operand1: "0".to_string(),
            operand2: "10".to_string(),
        }),
    )
}

#[test]
fn test_liquidator_left_in_unhealthy_state() {
    let uosmo_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(23654u128, 4).unwrap(),
        max_ltv: Decimal::from_atomics(5u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(55u128, 2).unwrap(),
    };
    let uatom_info = CoinInfo {
        denom: "uatom".to_string(),
        price: Decimal::from_atomics(102u128, 1).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(75u128, 2).unwrap(),
    };

    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(100u128))),
        ],
        &[Coin::new(300, uosmo_info.clone().denom)],
    )
    .unwrap();

    let health = mock.query_health(&liquidatee_token_id);
    assert!(!health.liquidatable);

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let liquidator = Addr::unchecked("liquidator");
    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![
            Borrow(uatom_info.to_coin(Uint128::from(10u128))),
            LiquidateCoin {
                liquidatee_token_id: liquidatee_token_id.clone(),
                debt: uatom_info.to_coin(Uint128::from(10u128)),
                request_coin: uosmo_info.to_coin(Uint128::from(50u128)),
            },
        ],
        &[],
    );

    assert_err(
        res,
        AboveMaxLTV {
            token_id: liquidator_token_id,
            max_ltv_health_factor: "0.295675".to_string(),
        },
    )
}

#[test]
fn test_healthier_assertion_when_fully_paid_off() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();
    let liquidator = Addr::unchecked("liquidator");
    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        .max_close_factor(Decimal::from_atomics(100u128, 0).unwrap())
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .fund_account(AccountToFund {
            addr: liquidator.clone(),
            funds: vec![Coin::new(300u128, uatom_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(100u128))),
        ],
        &[Coin::new(300, uosmo_info.clone().denom)],
    )
    .unwrap();

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![
            Deposit(uatom_info.to_coin(Uint128::from(101u128))),
            LiquidateCoin {
                liquidatee_token_id: liquidatee_token_id.clone(),
                debt: uatom_info.to_coin(Uint128::from(101u128)),
                request_coin: uosmo_info.to_coin(Uint128::from(50u128)),
            },
        ],
        &[uatom_info.to_coin(Uint128::from(101u128))],
    )
    .unwrap();

    let position = mock.query_position(&liquidatee_token_id);
    assert_eq!(position.debt_shares.len(), 0);
    let position = mock.query_health(&liquidatee_token_id);
    assert!(!position.liquidatable);
}

#[test]
fn test_liquidatee_not_healthier_after_liquidation() {
    let uosmo_info = CoinInfo {
        denom: "uosmo".to_string(),
        price: Decimal::from_atomics(23654u128, 4).unwrap(),
        max_ltv: Decimal::from_atomics(5u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(55u128, 2).unwrap(),
    };
    let uatom_info = CoinInfo {
        denom: "uatom".to_string(),
        price: Decimal::from_atomics(102u128, 1).unwrap(),
        max_ltv: Decimal::from_atomics(7u128, 1).unwrap(),
        liquidation_threshold: Decimal::from_atomics(75u128, 2).unwrap(),
    };
    let liquidator = Addr::unchecked("liquidator");
    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        // an absurdly high liquidation bonus
        .max_liquidation_bonus(Decimal::from_atomics(80u128, 0).unwrap())
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .fund_account(AccountToFund {
            addr: liquidator.clone(),
            funds: vec![Coin::new(300u128, uatom_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(100u128))),
        ],
        &[Coin::new(300, uosmo_info.denom)],
    )
    .unwrap();

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![
            Deposit(uatom_info.to_coin(Uint128::from(50u128))),
            LiquidateCoin {
                liquidatee_token_id: liquidatee_token_id.clone(),
                debt: uatom_info.to_coin(Uint128::from(50u128)),
                request_coin: uatom_info.to_coin(Uint128::from(100u128)),
            },
        ],
        &[uatom_info.to_coin(Uint128::from(50u128))],
    );

    assert_err(
        res,
        ContractError::HealthNotImproved {
            prev_hf: "0.935787623762376237".to_string(),
            new_hf: "0.56450049504950495".to_string(),
        },
    )
}

#[test]
fn test_requested_more_than_liq_bonus_rate() {}

#[test]
fn test_benevolent_loss() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();
    let liquidator = Addr::unchecked("liquidator");
    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![Coin::new(300u128, uosmo_info.denom.clone())],
        })
        .fund_account(AccountToFund {
            addr: liquidator.clone(),
            funds: vec![Coin::new(300u128, uatom_info.denom.clone())],
        })
        .build()
        .unwrap();
    let liquidatee_token_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_token_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(Uint128::from(300u128))),
            Borrow(uatom_info.to_coin(Uint128::from(100u128))),
        ],
        &[Coin::new(300, uosmo_info.clone().denom)],
    )
    .unwrap();

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let prev_position = mock.query_position(&liquidatee_token_id);
    let prev_health = mock.query_health(&liquidatee_token_id);
    let prev_total_debt_shares = mock.query_total_debt_shares("uatom");
    let prev_total_debt_amount = mock.query_red_bank_debt("uatom").amount;

    let liquidator_token_id = mock.create_credit_account(&liquidator).unwrap();

    mock.update_credit_account(
        &liquidator_token_id,
        &liquidator,
        vec![
            Deposit(uatom_info.to_coin(Uint128::from(10u128))),
            LiquidateCoin {
                liquidatee_token_id: liquidatee_token_id.clone(),
                debt: uatom_info.to_coin(Uint128::from(10u128)),
                request_coin: uosmo_info.to_coin(Uint128::from(50u128)),
            },
        ],
        &[uatom_info.to_coin(Uint128::from(10u128))],
    )
    .unwrap();

    // Liquidator got their reward in exchange for atom
    let position = mock.query_position(&liquidator_token_id);
    assert_eq!(position.coins.len(), 1);
    let repo_osmo = get_coin(&position.coins, "uosmo");
    assert_eq!(repo_osmo.amount, Uint128::from(50u128));
    assert_eq!(repo_osmo.denom, uosmo_info.denom);

    // Liquidatee's debt partially paid off
    let new_position = mock.query_position(&liquidatee_token_id);
    assert!(
        get_shares(&new_position.debt_shares, "uatom").shares
            < get_shares(&prev_position.debt_shares, "uatom").shares
    );
    assert_eq!(
        get_coin(&new_position.coins, "uosmo").amount,
        get_coin(&prev_position.coins, "uosmo")
            .amount
            .sub(Uint128::from(50u128)),
    );
    assert_eq!(
        get_coin(&new_position.coins, "uatom").amount,
        get_coin(&prev_position.coins, "uatom").amount
    );
    let new_health = mock.query_health(&liquidatee_token_id);
    assert!(new_health.lqdt_health_factor > prev_health.lqdt_health_factor);

    // assert total debt shares decremented
    let new_total_debt_shares = mock.query_total_debt_shares("uatom");
    assert!(new_total_debt_shares.shares < prev_total_debt_shares.shares);
    assert_eq!(
        new_total_debt_shares.shares,
        prev_total_debt_shares.shares.sub(
            prev_total_debt_shares
                .shares
                .multiply_ratio(Uint128::from(10u128), prev_total_debt_amount)
        )
    )
}

// REFERENCE: https://docs.google.com/spreadsheets/d/1_Bs1Fc1RLf5IARvaXZ0QjigoMWSJQhhrRUtQ8uyoLdI/edit?pli=1#gid=1857897311
// for price scenarios

// Debt coin:
// == Success cases ==
// close factor is minimum
// total debt is minimum
// liquidator choice is minimum

// Request coin:
// == Success cases ==
// liquidatee balance is minimum
// debt_adjusted_max is minimum
// liquidator choice is minimum
// == Failure ==
// all minimums cause too high of adjustment variance

// benevolence case

// TODO: After swap is implemented, attempt to liquidate with no deposited funds:
// - Borrow atom
// - Liquidate and collect osmo
// - Swap osmo for atom
// - Repay debt
// - Withdraw
#[test]
fn test_liquidate_with_no_deposited_funds() {}

fn get_coin(coins: &[CoinValue], denom: &str) -> CoinValue {
    coins
        .iter()
        .find(|coin| coin.denom.as_str() == denom)
        .unwrap()
        .clone()
}

fn get_shares(coins: &[DebtSharesValue], denom: &str) -> DebtSharesValue {
    coins
        .iter()
        .find(|coin| coin.denom.as_str() == denom)
        .unwrap()
        .clone()
}
