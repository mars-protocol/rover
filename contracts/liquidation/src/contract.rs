#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use mars_owner::OwnerInit::SetInitialOwner;
use mars_rover::{
    error::ContractResult,
    msg::liquidation::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
};

use crate::{
    compute::query_liquidation,
    state::{CREDIT_MANAGER, OWNER},
    update_config::update_config,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;

    OWNER.initialize(
        deps.storage,
        deps.api,
        SetInitialOwner {
            owner: msg.owner,
        },
    )?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        ExecuteMsg::UpdateConfig {
            credit_manager,
        } => update_config(deps, info, credit_manager),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> ContractResult<Binary> {
    let res = match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Liquidation {
            liquidatee_account_id,
            debt_coin,
            request_coin,
            liquidatee_debt_coin,
        } => to_binary(&query_liquidation(
            deps,
            liquidatee_account_id,
            debt_coin,
            request_coin,
            liquidatee_debt_coin,
        )?),
    };
    res.map_err(Into::into)
}

pub fn query_config(deps: Deps) -> ContractResult<ConfigResponse> {
    let credit_manager_addr = CREDIT_MANAGER.may_load(deps.storage)?.map(|a| a.into());
    let owner_response = OWNER.query(deps.storage)?;

    Ok(ConfigResponse {
        credit_manager_addr,
        owner_response,
    })
}
