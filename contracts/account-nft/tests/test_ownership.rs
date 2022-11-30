use cosmwasm_std::Addr;
use cw721_base::MinterResponse;

use mars_account_nft::msg::QueryMsg;

use crate::helpers::MockEnv;

pub mod helpers;

#[test]
fn test_only_owner_can_propose_ownership_transfer() {
    let mut mock = MockEnv::new().build().unwrap();

    let bad_guy = Addr::unchecked("bad_guy");
    let res = mock.propose_new_minter(&bad_guy, &bad_guy);

    if res.is_ok() {
        panic!("Non-owner should not be able to propose ownership transfer");
    }
}

#[test]
fn test_propose_ownership_stores() {
    let mut mock = MockEnv::new().build().unwrap();

    let new_minter = Addr::unchecked("new_minter");
    mock.propose_new_minter(&mock.minter.clone(), &new_minter)
        .unwrap();

    let config = mock.query_config();
    assert_eq!(config.proposed_new_minter.unwrap(), new_minter);
}

#[test]
fn test_proposed_owner_can_accept_ownership() {
    let mut mock = MockEnv::new().build().unwrap();

    let new_minter = Addr::unchecked("new_minter");
    mock.propose_new_minter(&mock.minter.clone(), &new_minter)
        .unwrap();

    mock.accept_proposed_minter(&new_minter).unwrap();

    let config = mock.query_config();
    if config.proposed_new_minter.is_some() {
        panic!("Proposed owner should have been removed from storage");
    }

    let res: MinterResponse = mock
        .app
        .wrap()
        .query_wasm_smart(mock.nft_contract, &QueryMsg::Minter {})
        .unwrap();

    assert_eq!(res.minter, new_minter)
}

#[test]
fn test_only_proposed_owner_can_accept() {
    let mut mock = MockEnv::new().build().unwrap();

    let new_minter = Addr::unchecked("new_minter");
    mock.propose_new_minter(&mock.minter.clone(), &new_minter)
        .unwrap();

    let bad_guy = Addr::unchecked("bad_guy");
    let res = mock.accept_proposed_minter(&bad_guy);

    if res.is_ok() {
        panic!("Only proposed owner can accept ownership");
    }
}
