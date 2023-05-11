#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use mars_rover::msg::QueryMsg;

use crate::{
    execute::{set_position_response, set_vault_config},
    msg::{ExecuteMsg, InstantiateMsg},
    query::{query_config, query_positions, query_vault_config},
    state::CONFIG,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    CONFIG.save(deps.storage, &msg.config)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SetPositionsResponse {
            account_id,
            positions,
        } => set_position_response(deps, account_id, positions),
        ExecuteMsg::SetVaultConfig {
            address,
            config,
        } => set_vault_config(deps, &address, config),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Positions {
            account_id,
        } => to_binary(&query_positions(deps, account_id)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::VaultConfig {
            vault,
        } => to_binary(&query_vault_config(deps, vault)?),
        _ => unimplemented!("query msg not supported"),
    }
}
