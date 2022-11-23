use cosmwasm_std::{Addr, Decimal, Empty, StdResult};
use cw721::NftInfoResponse;
use cw_multi_test::App;

use mars_account_nft::error::ContractError;
use mars_account_nft::error::ContractError::BurnNotAllowed;
use mars_account_nft::msg::QueryMsg::NftInfo;

use crate::helpers::{
    below_max_for_burn, burn_action, generate_health_response, get_token_id, mint_action, mock_env,
    set_health_response, MAX_VALUE_FOR_BURN,
};

pub mod helpers;

#[test]
fn test_burn_not_allowed_if_too_many_debts() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let user = Addr::unchecked("user");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id = get_token_id(res);

    set_health_response(
        &mut app,
        &user,
        &mock.credit_manager,
        &token_id,
        &generate_health_response(10_000, 0),
    );

    let res = burn_action(&mut app, &user, &mock.nft_contract, &token_id);
    let error: ContractError = res.unwrap_err().downcast().unwrap();
    assert_eq!(
        error,
        BurnNotAllowed {
            current_balances: Decimal::from_atomics(10_000u128, 0).unwrap(),
            max_value_allowed: Decimal::from_atomics(MAX_VALUE_FOR_BURN, 0).unwrap()
        }
    )
}

#[test]
fn test_burn_not_allowed_if_too_much_collateral() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let user = Addr::unchecked("user");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id = get_token_id(res);

    set_health_response(
        &mut app,
        &user,
        &mock.credit_manager,
        &token_id,
        &generate_health_response(0, 10_000),
    );

    let res = burn_action(&mut app, &user, &mock.nft_contract, &token_id);
    let error: ContractError = res.unwrap_err().downcast().unwrap();
    assert_eq!(
        error,
        BurnNotAllowed {
            current_balances: Decimal::from_atomics(10_000u128, 0).unwrap(),
            max_value_allowed: Decimal::from_atomics(MAX_VALUE_FOR_BURN, 0).unwrap()
        }
    )
}

#[test]
fn test_burn_allowance_works_with_both_debt_and_collateral() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let user = Addr::unchecked("user");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id = get_token_id(res);

    set_health_response(
        &mut app,
        &user,
        &mock.credit_manager,
        &token_id,
        &generate_health_response(501, 500),
    );

    let res = burn_action(&mut app, &user, &mock.nft_contract, &token_id);
    let error: ContractError = res.unwrap_err().downcast().unwrap();
    assert_eq!(
        error,
        BurnNotAllowed {
            current_balances: Decimal::from_atomics(1_001u128, 0).unwrap(),
            max_value_allowed: Decimal::from_atomics(MAX_VALUE_FOR_BURN, 0).unwrap()
        }
    )
}

#[test]
fn test_burn_allowance_at_exactly_max() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let user = Addr::unchecked("user");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id = get_token_id(res);

    set_health_response(
        &mut app,
        &user,
        &mock.credit_manager,
        &token_id,
        &generate_health_response(500, 500),
    );

    burn_action(&mut app, &user, &mock.nft_contract, &token_id).unwrap();
}

#[test]
fn test_burn_allowance_when_under_max() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let user = Addr::unchecked("user");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id = get_token_id(res);

    // Assert no errors on calling for NftInfo
    let _: NftInfoResponse<Empty> = app
        .wrap()
        .query_wasm_smart(
            mock.nft_contract.clone(),
            &NftInfo {
                token_id: token_id.clone(),
            },
        )
        .unwrap();

    set_health_response(
        &mut app,
        &user,
        &mock.credit_manager,
        &token_id,
        &below_max_for_burn(),
    );
    burn_action(&mut app, &user, &mock.nft_contract, &token_id).unwrap();
    let res: StdResult<NftInfoResponse<Empty>> = app
        .wrap()
        .query_wasm_smart(mock.nft_contract, &NftInfo { token_id });
    res.unwrap_err();
}
