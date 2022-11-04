use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use mars_rover::adapters::swap::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg<Empty>,
    }
}
