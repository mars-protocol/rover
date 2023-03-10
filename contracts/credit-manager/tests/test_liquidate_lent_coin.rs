use cosmwasm_std::{coins, Addr, Decimal, Uint128};
use mars_mock_oracle::msg::CoinPrice;
use mars_rover::{
    error::{ContractError, ContractError::NotLiquidatable},
    msg::execute::Action::{Borrow, Deposit, Lend, LiquidateCoin},
};

use crate::helpers::{
    assert_err, get_coin, get_debt, get_lent, uatom_info, ujake_info, uosmo_info, AccountToFund,
    MockEnv,
};

pub mod helpers;

#[test]
fn lent_positions_contribute_to_health() {
    let uatom_info = uatom_info();
    let uosmo_info = uosmo_info();

    let liquidatee = Addr::unchecked("liquidatee");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uatom_info.clone(), uosmo_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![uatom_info.to_coin(500), uosmo_info.to_coin(500)],
        })
        .build()
        .unwrap();

    let liquidatee_account_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_account_id,
        &liquidatee,
        vec![Deposit(uatom_info.to_coin(100)), Borrow(uosmo_info.to_coin(40))],
        &[uatom_info.to_coin(100)],
    )
    .unwrap();

    let health_1 = mock.query_health(&liquidatee_account_id);
    assert!(!health_1.liquidatable);

    mock.update_credit_account(
        &liquidatee_account_id,
        &liquidatee,
        vec![Lend(uatom_info.to_coin(50))],
        &[],
    )
    .unwrap();

    // Collateral should be the same after Lend
    let health_2 = mock.query_health(&liquidatee_account_id);
    assert!(!health_2.liquidatable);
    // health_2.total_collateral_value bigger (+1) because of simulated yield
    assert_eq!(health_1.total_collateral_value, health_2.total_collateral_value - Uint128::one());
    assert_eq!(health_1.max_ltv_adjusted_collateral, health_2.max_ltv_adjusted_collateral);
    assert_eq!(
        health_1.liquidation_threshold_adjusted_collateral,
        health_2.liquidation_threshold_adjusted_collateral
    );

    let liquidator = Addr::unchecked("liquidator");
    let liquidator_account_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_account_id,
        &liquidator,
        vec![LiquidateCoin {
            liquidatee_account_id: liquidatee_account_id.clone(),
            debt_coin: uosmo_info.to_coin(10),
            request_coin_denom: uatom_info.denom,
        }],
        &[],
    );

    assert_err(
        res,
        NotLiquidatable {
            account_id: liquidatee_account_id,
            lqdt_health_factor: "9.7".to_string(),
        },
    )
}

#[test]
fn liquidatee_does_not_have_requested_lent_coin() {
    let uatom_info = uatom_info();
    let uosmo_info = uosmo_info();
    let ujake_info = ujake_info();

    let liquidatee = Addr::unchecked("liquidatee");
    let liquidator = Addr::unchecked("liquidator");

    let mut mock = MockEnv::new()
        .allowed_coins(&[uatom_info.clone(), uosmo_info.clone(), ujake_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: vec![uatom_info.to_coin(500)],
        })
        .fund_account(AccountToFund {
            addr: liquidator.clone(),
            funds: vec![uosmo_info.to_coin(500)],
        })
        .build()
        .unwrap();

    let liquidatee_account_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_account_id,
        &liquidatee,
        vec![
            Deposit(uatom_info.to_coin(100)),
            Lend(uatom_info.to_coin(50)),
            Borrow(uosmo_info.to_coin(100)),
        ],
        &[uatom_info.to_coin(100)],
    )
    .unwrap();

    mock.price_change(CoinPrice {
        denom: uosmo_info.denom.clone(),
        price: Decimal::from_atomics(20u128, 0).unwrap(),
    });

    let health = mock.query_health(&liquidatee_account_id);
    assert!(health.liquidatable);

    let liquidator_account_id = mock.create_credit_account(&liquidator).unwrap();

    let res = mock.update_credit_account(
        &liquidator_account_id,
        &liquidator,
        vec![
            Deposit(uosmo_info.to_coin(10)),
            LiquidateCoin {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uosmo_info.to_coin(10),
                request_coin_denom: ujake_info.denom.clone(),
            },
        ],
        &[uosmo_info.to_coin(10)],
    );

    assert_err(res, ContractError::CoinNotAvailable(ujake_info.denom));
}

#[test]
fn liquidate_without_reclaiming() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();

    let liquidator = Addr::unchecked("liquidator");
    let liquidatee = Addr::unchecked("liquidatee");

    let mut mock = MockEnv::new()
        .max_close_factor(Decimal::from_atomics(1u128, 1).unwrap())
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: coins(300, uosmo_info.denom.clone()),
        })
        .fund_account(AccountToFund {
            addr: liquidator.clone(),
            funds: coins(300, uatom_info.denom.clone()),
        })
        .build()
        .unwrap();

    let liquidatee_account_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_account_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(300)),
            Borrow(uatom_info.to_coin(100)),
            Lend(uosmo_info.to_coin(52)),
        ],
        &[uosmo_info.to_coin(300)],
    )
    .unwrap();

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(55u128, 1).unwrap(),
    });

    let health = mock.query_health(&liquidatee_account_id);
    assert!(health.liquidatable);
    assert_eq!(health.total_collateral_value, Uint128::new(625u128)); // 300 * 0.25 + 100 * 5.5
    assert_eq!(health.total_debt_value, Uint128::new(555u128)); // (100 + 1) * 5.5

    let liquidator_account_id = mock.create_credit_account(&liquidator).unwrap();

    mock.update_credit_account(
        &liquidator_account_id,
        &liquidator,
        vec![
            Deposit(uatom_info.to_coin(10)),
            LiquidateCoin {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uatom_info.to_coin(10),
                request_coin_denom: uosmo_info.denom,
            },
        ],
        &[uatom_info.to_coin(10)],
    )
    .unwrap();

    // Assert liquidatee's new position
    let position = mock.query_positions(&liquidatee_account_id);
    assert_eq!(position.deposits.len(), 2);
    let osmo_balance = get_coin("uosmo", &position.deposits);
    assert_eq!(osmo_balance.amount, Uint128::new(8)); // requested collateral: floor(10 * 5.5 * 1.1) / 0.25 = 240, deposit: 248, lent: 52
    let atom_balance = get_coin("uatom", &position.deposits);
    assert_eq!(atom_balance.amount, Uint128::new(100));

    assert_eq!(position.debts.len(), 1);
    let atom_debt = get_debt("uatom", &position.debts);
    assert_eq!(atom_debt.amount, Uint128::new(91));

    assert_eq!(position.lends.len(), 1);
    let osmo_lent = get_lent("uosmo", &position.lends);
    assert_eq!(osmo_lent.amount, Uint128::new(53));

    // Assert liquidator's new position
    let position = mock.query_positions(&liquidator_account_id);
    assert_eq!(position.deposits.len(), 1);
    assert_eq!(position.debts.len(), 0);
    let osmo_balance = get_coin("uosmo", &position.deposits);
    assert_eq!(osmo_balance.amount, Uint128::new(240));
}

#[test]
fn liquidate_with_reclaiming() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();

    let liquidator = Addr::unchecked("liquidator");
    let liquidatee = Addr::unchecked("liquidatee");

    let mut mock = MockEnv::new()
        .max_close_factor(Decimal::from_atomics(1u128, 1).unwrap())
        .allowed_coins(&[uosmo_info.clone(), uatom_info.clone()])
        .fund_account(AccountToFund {
            addr: liquidatee.clone(),
            funds: coins(300, uosmo_info.denom.clone()),
        })
        .fund_account(AccountToFund {
            addr: liquidator.clone(),
            funds: coins(300, uatom_info.denom.clone()),
        })
        .build()
        .unwrap();

    let liquidatee_account_id = mock.create_credit_account(&liquidatee).unwrap();

    mock.update_credit_account(
        &liquidatee_account_id,
        &liquidatee,
        vec![
            Deposit(uosmo_info.to_coin(300)),
            Borrow(uatom_info.to_coin(100)),
            Lend(uosmo_info.to_coin(80)),
        ],
        &[uosmo_info.to_coin(300)],
    )
    .unwrap();

    mock.price_change(CoinPrice {
        denom: uatom_info.denom.clone(),
        price: Decimal::from_atomics(55u128, 1).unwrap(),
    });

    let health = mock.query_health(&liquidatee_account_id);
    assert!(health.liquidatable);
    assert_eq!(health.total_collateral_value, Uint128::new(625u128)); // 300 * 0.25 + 100 * 5.5
    assert_eq!(health.total_debt_value, Uint128::new(555u128)); // (100 + 1) * 5.5

    let liquidator_account_id = mock.create_credit_account(&liquidator).unwrap();

    mock.update_credit_account(
        &liquidator_account_id,
        &liquidator,
        vec![
            Deposit(uatom_info.to_coin(10)),
            LiquidateCoin {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uatom_info.to_coin(10),
                request_coin_denom: uosmo_info.denom,
            },
        ],
        &[uatom_info.to_coin(10)],
    )
    .unwrap();

    // Assert liquidatee's new position
    let position = mock.query_positions(&liquidatee_account_id);
    assert_eq!(position.deposits.len(), 1); // uosmo deposit is fully liquidated
    let atom_balance = get_coin("uatom", &position.deposits);
    assert_eq!(atom_balance.amount, Uint128::new(100));

    assert_eq!(position.debts.len(), 1);
    let atom_debt = get_debt("uatom", &position.debts);
    assert_eq!(atom_debt.amount, Uint128::new(91));

    // requested collateral: floor(10 * 5.5 * 1.1) / 0.25 = 240
    // deposit: 220
    // lent: 80
    //
    // 20 usomo should be reclaimed
    // 80 - 20 = 60 (add +1 because of simulated yield so it should be 61)
    assert_eq!(position.lends.len(), 1);
    let osmo_lent = get_lent("uosmo", &position.lends);
    assert_eq!(osmo_lent.amount, Uint128::new(61));

    // Assert liquidator's new position
    let position = mock.query_positions(&liquidator_account_id);
    assert_eq!(position.deposits.len(), 1);
    assert_eq!(position.debts.len(), 0);
    let osmo_balance = get_coin("uosmo", &position.deposits);
    assert_eq!(osmo_balance.amount, Uint128::new(240));
}
