use std::fmt::Display;
use std::str::FromStr;

use osmosis_testing::cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use osmosis_testing::{Account, Bank, OsmosisTestApp, RunnerError, SigningAccount, Wasm};

use rover::adapters::swap::InstantiateMsg;

const ARTIFACTS_DIR_PATH: &str = env!("ARTIFACTS_DIR_PATH");
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");

pub fn wasm_file() -> String {
    let snaked_name = CONTRACT_NAME.replace('-', "_");
    format!("../../../{}/{}.wasm", ARTIFACTS_DIR_PATH, snaked_name)
}

pub fn instantiate_contract(wasm: &Wasm<OsmosisTestApp>, owner: &SigningAccount) -> String {
    let wasm_byte_code = std::fs::read(wasm_file()).unwrap();
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
    bank.query_balance(&QueryBalanceRequest {
        address: addr.to_string(),
        denom: denom.to_string(),
    })
    .unwrap()
    .balance
    .map(|c| u128::from_str(&c.amount).unwrap())
    .unwrap_or(0)
}

pub fn assert_err(actual: RunnerError, expected: impl Display) {
    match actual {
        RunnerError::ExecuteError { msg } => {
            assert!(msg.contains(&format!("{}", expected)))
        }
        RunnerError::QueryError { msg } => {
            assert!(msg.contains(&format!("{}", expected)))
        }
        _ => panic!("Unhandled error"),
    }
}
