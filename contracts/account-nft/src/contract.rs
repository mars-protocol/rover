use std::convert::TryInto;

use crate::error::ContractError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw721_base::Cw721Contract;

use crate::execute::{accept_ownership, burn, mint, propose_new_owner};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_proposed_new_owner;
use crate::state::{CREDIT_MANAGER, MAX_VALUE_FOR_BURN, NEXT_ID};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Extending CW721 base contract
pub type Parent<'a> = Cw721Contract<'a, Empty, Empty, Empty, Empty>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(
        deps.storage,
        &format!("crates.io:{}", CONTRACT_NAME),
        CONTRACT_VERSION,
    )?;
    NEXT_ID.save(deps.storage, &1)?;

    let cm_addr = deps.api.addr_validate(&msg.credit_manager)?;
    CREDIT_MANAGER.save(deps.storage, &cm_addr)?;

    MAX_VALUE_FOR_BURN.save(deps.storage, &msg.max_value_for_burn)?;

    Parent::default().instantiate(deps, env, info, msg.into())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { user } => mint(deps, env, info, &user),
        ExecuteMsg::ProposeNewOwner { new_owner } => propose_new_owner(deps, info, &new_owner),
        ExecuteMsg::AcceptOwnership {} => accept_ownership(deps, info),
        ExecuteMsg::Burn { token_id } => burn(deps, env, info, token_id),
        _ => Parent::default()
            .execute(deps, env, info, msg.try_into()?)
            .map_err(Into::into),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::ProposedNewOwner {} => to_binary(&query_proposed_new_owner(deps)?),
        _ => Parent::default().query(deps, env, msg.try_into()?),
    }
}
