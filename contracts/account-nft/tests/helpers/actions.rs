use anyhow::Result as AnyResult;
use cosmwasm_std::Addr;
use cw_multi_test::{AppResponse, BasicApp, Executor};

use mars_account_nft::msg::ExecuteMsg as ExtendedExecuteMsg;
use mars_mock_credit_manager::msg::ExecuteMsg::SetHealthResponse;
use mars_rover::msg::query::HealthResponse;

pub fn set_health_response(
    app: &mut BasicApp,
    sender: &Addr,
    credit_manager_addr: &Addr,
    account_id: &str,
    response: &HealthResponse,
) -> AppResponse {
    app.execute_contract(
        sender.clone(),
        credit_manager_addr.clone(),
        &SetHealthResponse {
            account_id: account_id.to_string(),
            response: response.clone(),
        },
        &[],
    )
    .unwrap()
}

pub fn mint_action(
    app: &mut BasicApp,
    sender: &Addr,
    contract_addr: &Addr,
    token_owner: &Addr,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        contract_addr.clone(),
        &ExtendedExecuteMsg::Mint {
            user: token_owner.into(),
        },
        &[],
    )
}

pub fn burn_action(
    app: &mut BasicApp,
    sender: &Addr,
    contract_addr: &Addr,
    token_id: &str,
) -> AnyResult<AppResponse> {
    app.execute_contract(
        sender.clone(),
        contract_addr.clone(),
        &ExtendedExecuteMsg::Burn {
            token_id: token_id.to_string(),
        },
        &[],
    )
}
