use std::fmt::Error;

use cosmwasm_std::Addr;
use cw721::OwnerOfResponse;
use cw721_base::ContractError::Unauthorized;
use cw_multi_test::{App, Executor};

use mars_account_nft::error::ContractError;
use mars_account_nft::error::ContractError::BaseError;
use mars_account_nft::msg::ExecuteMsg as ExtendedExecuteMsg;
use mars_account_nft::msg::QueryMsg::OwnerOf;

use crate::helpers::{
    assert_owner_is_correct, below_max_for_burn, burn_action, get_token_id, mint_action, mock_env,
    set_health_response,
};

pub mod helpers;

#[test]
fn test_id_incrementer() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let user_1 = Addr::unchecked("user_1");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user_1).unwrap();
    let token_id = get_token_id(res);
    assert_eq!(token_id, "1");
    assert_owner_is_correct(&mut app, &mock.nft_contract, &user_1, &token_id);

    let user_2 = Addr::unchecked("user_2");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user_2).unwrap();
    let token_id = get_token_id(res);
    assert_eq!(token_id, "2");
    assert_owner_is_correct(&mut app, &mock.nft_contract, &user_2, &token_id);

    let user_3 = Addr::unchecked("user_3");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user_3).unwrap();
    let token_id = get_token_id(res);
    assert_eq!(token_id, "3");
    assert_owner_is_correct(&mut app, &mock.nft_contract, &user_3, &token_id);
}

#[test]
fn test_id_incrementer_works_despite_burns() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let user = Addr::unchecked("user");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id_1 = get_token_id(res);
    set_health_response(
        &mut app,
        &user,
        &mock.credit_manager,
        &token_id_1,
        &below_max_for_burn(),
    );
    assert_eq!(token_id_1, "1");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id_2 = get_token_id(res);
    set_health_response(
        &mut app,
        &user,
        &mock.credit_manager,
        &token_id_2,
        &below_max_for_burn(),
    );
    assert_eq!(token_id_2, "2");

    burn_action(&mut app, &user, &mock.nft_contract, &token_id_1).unwrap();
    burn_action(&mut app, &user, &mock.nft_contract, &token_id_2).unwrap();

    let res = mint_action(&mut app, &owner, &mock.nft_contract, &user).unwrap();
    let token_id = get_token_id(res);
    assert_eq!(token_id, "3");
    assert_owner_is_correct(&mut app, &mock.nft_contract, &user, &token_id);
}

#[test]
fn test_only_contract_owner_can_mint() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let bad_guy = Addr::unchecked("bad_guy");
    let res = mint_action(&mut app, &bad_guy, &mock.nft_contract, &bad_guy);
    let err: ContractError = res.unwrap_err().downcast().unwrap();
    assert_eq!(err, BaseError(Unauthorized {}))
}

#[test]
fn test_only_token_owner_can_burn() {
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
        &below_max_for_burn(),
    );

    let bad_guy = Addr::unchecked("bad_guy");
    let res = burn_action(&mut app, &bad_guy, &mock.nft_contract, &token_id);
    let err: ContractError = res.unwrap_err().downcast().unwrap();
    assert_eq!(err, BaseError(Unauthorized {}));

    burn_action(&mut app, &user, &mock.nft_contract, &token_id).unwrap();
}

#[test]
fn test_normal_base_cw721_actions_can_still_be_taken() {
    let mut app = App::default();
    let owner = Addr::unchecked("owner");
    let mock = mock_env(&mut app, &owner);

    let rover_user_a = Addr::unchecked("rover_user_a");
    let res = mint_action(&mut app, &owner, &mock.nft_contract, &rover_user_a).unwrap();
    let token_id = get_token_id(res);

    let rover_user_b = Addr::unchecked("rover_user_b");
    let transfer_msg: ExtendedExecuteMsg = ExtendedExecuteMsg::TransferNft {
        token_id: token_id.clone(),
        recipient: rover_user_b.clone().into(),
    };
    app.execute_contract(rover_user_a, mock.nft_contract.clone(), &transfer_msg, &[])
        .map_err(|_| Error::default())
        .unwrap();

    let res: OwnerOfResponse = app
        .wrap()
        .query_wasm_smart(
            mock.nft_contract,
            &OwnerOf {
                token_id,
                include_expired: None,
            },
        )
        .unwrap();
    assert_eq!(res.owner, rover_user_b.to_string())
}
