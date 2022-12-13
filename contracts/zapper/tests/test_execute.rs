use cosmwasm_std::{coin, Uint128};
use osmosis_testing::{Gamm, Module, OsmosisTestApp, Wasm};

use mars_zapper::msg::{ExecuteMsg, QueryMsg};

use crate::helpers::instantiate_contract;

pub mod helpers;

#[test]
fn test_provide_liquidity() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app
        .init_account(&[
            coin(1_000_000_000_000, "uatom"),
            coin(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let contract_addr = instantiate_contract(&wasm, &signer);

    let gamm = Gamm::new(&app);
    let _pool_id = gamm
        .create_basic_pool(
            &[coin(2_000_000, "uatom"), coin(4_000_000, "uosmo")],
            &signer,
        )
        .unwrap()
        .data
        .pool_id;

    let res: Uint128 = wasm
        .query(
            &contract_addr,
            &QueryMsg::EstimateProvideLiquidity {
                lp_token_out: "gamm/pool/1".to_string(),
                coins_in: vec![coin(500_000, "uatom"), coin(2_000_000, "uosmo")],
            },
        )
        .unwrap();

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::ProvideLiquidity {
            lp_token_out: "gamm/pool/1".to_string(),
            recipient: None,
            minimum_receive: res,
        },
        &[coin(550_000, "uatom"), coin(6_500_000, "uosmo")],
        &signer,
    )
    .unwrap();
}
