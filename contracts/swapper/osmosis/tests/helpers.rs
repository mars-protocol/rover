use std::str::FromStr;

use osmosis_testing::{Account, Bank, OsmosisTestApp, RunnerError, SigningAccount, Wasm};

use rover::adapters::swap::InstantiateMsg;
use swapper_base::ContractError;

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

pub fn query_balance(bank: &Bank<OsmosisTestApp>, addr: &str, denom: &str) -> u128 {
    bank.query_all_balances(&addr, None)
        .unwrap()
        .balances
        .iter()
        .find(|b| b.denom == denom)
        .map(|b| u128::from_str(&b.amount).unwrap())
        .unwrap_or(0)
}

pub fn assert_contract_err(exec_err: RunnerError, expected_err: ContractError) {
    match exec_err {
        RunnerError::ExecuteError { msg } => {
            assert!(msg.contains(&format!("{}", expected_err)))
        }
        RunnerError::QueryError { msg } => {
            assert!(msg.contains(&format!("{}", expected_err)))
        }
        _ => panic!("Unhandled error"),
    }
}

pub fn assert_string_err(exec_err: RunnerError, expected_err: &str) {
    match exec_err {
        RunnerError::ExecuteError { msg } => {
            assert!(msg.contains(&format!("{}", expected_err)))
        }
        RunnerError::QueryError { msg } => {
            assert!(msg.contains(&format!("{}", expected_err)))
        }
        _ => panic!("Unhandled error"),
    }
}
