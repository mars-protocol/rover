use cosmwasm_std::{Addr, StdError, Uint128};
use mars_owner::OwnerError::NotOwner;
use mars_rover::{
    adapters::vault::VaultUnchecked,
    error::ContractError::{Owner, Std},
    msg::execute::Action::{Deposit, EnterVault, RequestVaultUnlock},
};

use crate::helpers::{
    assert_err, locked_vault_info, lp_token_info, unlocked_vault_info, AccountToFund, MockEnv,
};

pub mod helpers;

#[test]
fn only_owner_can_update_lockup_ids() {
    let mut mock = MockEnv::new().build().unwrap();

    let bad_guy = Addr::unchecked("bad_guy");
    let res =
        mock.update_lockup_id(&bad_guy, "12312", &VaultUnchecked::new("xyz".to_string()), 12, 33);

    assert_err(res, Owner(NotOwner {}))
}

#[test]
fn raises_when_no_position_matches() {
    let lp_token = lp_token_info();
    let leverage_vault = unlocked_vault_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[lp_token.clone()])
        .vault_configs(&[leverage_vault.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![lp_token.to_coin(300)],
        })
        .build()
        .unwrap();

    let vault = mock.get_vault(&leverage_vault);
    let account_id = mock.create_credit_account(&user).unwrap();

    mock.update_credit_account(
        &account_id,
        &user,
        vec![
            Deposit(lp_token.to_coin(200)),
            EnterVault {
                vault: vault.clone(),
                coin: lp_token.to_action_coin(23),
            },
        ],
        &[lp_token.to_coin(200)],
    )
    .unwrap();

    let res = mock.update_lockup_id(&Addr::unchecked("owner"), &account_id, &vault, 12, 33);

    assert_err(res, Std(StdError::generic_err("Lockup id: 12 not found for account id")));
}

#[test]
fn successful_lockup_id_update() {
    let lp_token = lp_token_info();
    let leverage_vault = locked_vault_info();

    let user = Addr::unchecked("user");
    let mut mock = MockEnv::new()
        .allowed_coins(&[lp_token.clone()])
        .vault_configs(&[leverage_vault.clone()])
        .fund_account(AccountToFund {
            addr: user.clone(),
            funds: vec![lp_token.to_coin(200)],
        })
        .build()
        .unwrap();

    let vault = mock.get_vault(&leverage_vault);
    let account_id = mock.create_credit_account(&user).unwrap();

    mock.update_credit_account(
        &account_id,
        &user,
        vec![
            Deposit(lp_token.to_coin(200)),
            EnterVault {
                vault: vault.clone(),
                coin: lp_token.to_action_coin(200),
            },
            RequestVaultUnlock {
                vault: vault.clone(),
                amount: Uint128::new(100_000),
            },
            RequestVaultUnlock {
                vault: vault.clone(),
                amount: Uint128::new(250_000),
            },
            RequestVaultUnlock {
                vault: vault.clone(),
                amount: Uint128::new(50_000),
            },
        ],
        &[lp_token.to_coin(200)],
    )
    .unwrap();

    let res = mock.query_positions(&account_id);
    assert_eq!(res.vaults.len(), 1);
    let position_amount = &res.vaults.first().unwrap().amount;
    let original_locked_amount = position_amount.locked();
    let original_unlocking = position_amount.unlocking().positions();
    assert_eq!(original_unlocking.len(), 3);
    assert_eq!(original_unlocking.get(0).unwrap().id, 1);

    let lockup_to_replace = original_unlocking.get(1).unwrap();
    assert_eq!(lockup_to_replace.id, 2);

    assert_eq!(original_unlocking.get(2).unwrap().id, 3);

    mock.update_lockup_id(&Addr::unchecked("owner"), &account_id, &vault, 2, 88).unwrap();

    let res = mock.query_positions(&account_id);
    assert_eq!(res.vaults.len(), 1);
    let position_amount = &res.vaults.first().unwrap().amount;
    assert_eq!(original_locked_amount, position_amount.locked());
    let new_unlocking = position_amount.unlocking().positions();
    assert_eq!(new_unlocking.len(), 3);
    assert_eq!(new_unlocking.get(0).unwrap().id, 1);
    assert_eq!(new_unlocking.get(1).unwrap().id, 3);

    let replaced_lockup = new_unlocking.get(2).unwrap();
    assert_eq!(replaced_lockup.id, 88);
    assert_eq!(lockup_to_replace.coin.amount, replaced_lockup.coin.amount);
    assert_eq!(lockup_to_replace.coin.denom, replaced_lockup.coin.denom);
}
