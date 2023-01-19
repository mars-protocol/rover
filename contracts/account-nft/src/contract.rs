use std::convert::TryInto;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw721::ContractInfoResponse;
use cw721_base::Cw721Contract;
use mars_rover::nft_config::NftConfig;

use crate::{
    error::ContractError,
    execute::{accept_minter_role, burn, mint, update_config},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::{query_config, query_next_id},
    state::{CONFIG, NEXT_ID},
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Extending CW721 base contract
pub type Parent<'a> = Cw721Contract<'a, Empty, Empty, Empty, Empty>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;

    NEXT_ID.save(deps.storage, &1)?;

    CONFIG.save(
        deps.storage,
        &NftConfig {
            max_value_for_burn: msg.max_value_for_burn,
            proposed_new_minter: None,
        },
    )?;

    // Parent::default().instantiate() copied below
    // Cannot use given it overrides contract version
    let info = ContractInfoResponse {
        name: msg.name,
        symbol: msg.symbol,
    };
    Parent::default().contract_info.save(deps.storage, &info)?;
    let minter = deps.api.addr_validate(&msg.minter)?;
    Parent::default().minter.save(deps.storage, &minter)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {
            user,
        } => mint(deps, env, info, &user),
        ExecuteMsg::UpdateConfig {
            updates,
        } => update_config(deps, info, updates),
        ExecuteMsg::AcceptMinterRole {} => accept_minter_role(deps, info),
        ExecuteMsg::Burn {
            token_id,
        } => burn(deps, env, info, token_id),
        _ => Parent::default().execute(deps, env, info, msg.try_into()?).map_err(Into::into),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::NextId {} => to_binary(&query_next_id(deps)?),
        _ => Parent::default().query(deps, env, msg.try_into()?),
    }
}
