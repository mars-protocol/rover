use cosmwasm_std::{coin, Addr, Coin, Uint128};
use osmosis_testing::{Account, Module, OsmosisTestApp, Wasm};

use mars_zapper_base::{CallbackMsg, ContractError, ExecuteMsg};

use crate::helpers::{assert_err, instantiate_contract};

pub mod helpers;

#[test]
fn test_only_contract_itself_can_callback() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let accs = app
        .init_accounts(
            &[
                coin(1_000_000_000_000, "uatom"),
                coin(1_000_000_000_000, "uosmo"),
            ],
            2,
        )
        .unwrap();
    let admin = &accs[0];
    let user = &accs[1];

    let contract_addr = instantiate_contract(&wasm, admin);

    let res_err = wasm
        .execute(
            &contract_addr,
            &ExecuteMsg::Callback(CallbackMsg::ReturnLpTokens {
                balance_before: Coin::new(1u128, "gamm/pool/1"),
                recipient: Addr::unchecked(user.address()),
                minimum_receive: Uint128::one(),
            }),
            &[],
            user,
        )
        .unwrap_err();
    assert_err(res_err, ContractError::Unauthorized {});
}
