use cosmwasm_schema::write_api;
use cosmwasm_vault_standard::msg::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
use mars_mock_vault::msg::InstantiateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: VaultStandardQueryMsg,
        execute: VaultStandardExecuteMsg,
    }
}
