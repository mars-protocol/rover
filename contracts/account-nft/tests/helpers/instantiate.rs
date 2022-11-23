use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Empty};
use cw_multi_test::{BasicApp, Executor};

use mars_account_nft::msg::InstantiateMsg;

use crate::helpers::{mock_credit_manager_contract, mock_nft_contract, MAX_VALUE_FOR_BURN};

#[cw_serde]
pub struct MockEnv {
    pub credit_manager: Addr,
    pub nft_contract: Addr,
}

pub fn mock_env(app: &mut BasicApp, owner: &Addr) -> MockEnv {
    let contract = mock_nft_contract();
    let code_id = app.store_code(contract);
    let credit_manager = instantiate_mock_credit_manager_contract(app, owner);

    let nft_contract = app
        .instantiate_contract(
            code_id,
            owner.clone(),
            &InstantiateMsg {
                credit_manager: credit_manager.clone().into(),
                max_value_for_burn: Decimal::from_atomics(MAX_VALUE_FOR_BURN, 0).unwrap(),
                name: "mock_nft".to_string(),
                symbol: "MOCK".to_string(),
                minter: owner.to_string(),
            },
            &[],
            "mock-account-nft",
            None,
        )
        .unwrap();

    MockEnv {
        credit_manager,
        nft_contract,
    }
}

pub fn instantiate_mock_credit_manager_contract(app: &mut BasicApp, owner: &Addr) -> Addr {
    let contract = mock_credit_manager_contract();
    let code_id = app.store_code(contract);

    app.instantiate_contract(
        code_id,
        owner.clone(),
        &Empty {},
        &[],
        "mock-credit-manager",
        None,
    )
    .unwrap()
}
