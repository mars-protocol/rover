use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

use mock_vault::contract::STARTING_SHARES;
use rover::msg::execute::Action::{Deposit, VaultDeposit};

use crate::helpers::{AccountToFund, CoinInfo, MockEnv, VaultTestInfo};

pub mod helpers;

#[test]
fn test_deposit_into_unlocked_vault() {
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

    let leverage_vault = VaultTestInfo {
        lp_token_denom: "uleverage".to_string(),
        lockup: None,
        unlock_request_queue: None,
        asset_denoms: vec!["uatom".to_string(), "uosmo".to_string()],
    };

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[uatom.clone(), uosmo.clone()])
        .allowed_vaults(&[leverage_vault.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![Coin::new(300u128, "uatom"), Coin::new(500u128, "uosmo")],
        })
        .build()
        .unwrap();

    let vault = mock.get_vault(&leverage_vault);
    let token_id = mock.create_credit_account(&user).unwrap();

    mock.update_credit_account(
        &token_id,
        &user,
        vec![
            Deposit(Coin {
                denom: uatom.denom,
                amount: Uint128::from(200u128),
            }),
            Deposit(Coin {
                denom: uosmo.denom,
                amount: Uint128::from(400u128),
            }),
            VaultDeposit {
                vault: vault.clone(),
                assets: vec![Coin::new(23u128, "uatom"), Coin::new(120u128, "uosmo")],
            },
        ],
        &[Coin::new(200u128, "uatom"), Coin::new(400u128, "uosmo")],
    )
    .unwrap();

    let lp_balance = mock.query_balance(&mock.rover, &leverage_vault.lp_token_denom);
    assert_eq!(STARTING_SHARES, lp_balance.amount);

    let res = mock.query_position(&token_id);
    assert_eq!(res.vault_positions.len(), 1);
    assert_eq!(
        STARTING_SHARES,
        res.vault_positions.first().unwrap().position.unlocked
    );
    assert_eq!(
        Uint128::zero(),
        res.vault_positions.first().unwrap().position.locked
    );

    let assets = mock.query_preview_redeem(
        &vault,
        res.vault_positions.first().unwrap().position.unlocked,
    );

    let osmo_withdraw = assets.iter().find(|coin| coin.denom == "uosmo").unwrap();
    assert_eq!(osmo_withdraw.amount, Uint128::from(120u128));
    let atom_withdraw = assets.iter().find(|coin| coin.denom == "uatom").unwrap();
    assert_eq!(atom_withdraw.amount, Uint128::from(23u128));
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
