use std::fmt::Debug;

use anyhow::Result as AnyResult;
use cosmwasm_std::Addr;
use cosmwasm_std::CustomQuery;
use cw_multi_test::{AppResponse, Executor};
use cw_multi_test::{Contract, ContractWrapper};
use osmosis_testing::{Account, Module, OsmosisTestApp, RunnerError, SigningAccount, Wasm};
use schemars::JsonSchema;

use rover::adapters::swap::{Config, ExecuteMsg, InstantiateMsg, QueryMsg};
use swapper_base::ContractError;
use swapper_osmosis::contract::{execute, instantiate, query};
use swapper_osmosis::route::OsmosisRoute;

pub fn instantiate_contract(wasm: &Wasm<OsmosisTestApp>, owner: &SigningAccount) -> String {
    let wasm_byte_code = std::fs::read("../../../artifacts/swapper_osmosis.wasm").unwrap();
    let code_id = wasm
        .store_code(&wasm_byte_code, None, owner)
        .unwrap()
        .data
        .code_id;

    wasm.instantiate(
        code_id,
        &InstantiateMsg {
            owner: owner.address(),
        },
        None,
        Some("swapper-osmosis-contract"),
        &[],
        owner,
    )
    .unwrap()
    .data
    .address
}

pub fn assert_err(exec_err: RunnerError, expected_err: ContractError) {
    match exec_err {
        RunnerError::ExecuteError { msg } => {
            assert!(msg.contains(&format!("{}", expected_err)))
        }
        _ => panic!("Unhandled error"),
    }
}
