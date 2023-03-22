#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use mars_owner::OwnerInit::SetInitialOwner;
use mars_rover::{error::ContractResult, msg::execute::LiquidateRequest};

use crate::{
    compute::{liquidate_deposit, liquidate_lend, liquidate_vault},
    msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg},
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        ExecuteMsg::UpdateConfig {
            credit_manager,
        } => update_config(deps, info, credit_manager),
        ExecuteMsg::Liquidate {
            liquidator_account_id,
            liquidatee_account_id,
            debt_coin,
            request,
        } => {
            // FIXME: check ownership of the account
            // assert_is_token_owner(&deps, &info.sender, account_id)?;
            match request {
                LiquidateRequest::Deposit(request_coin_denom) => liquidate_deposit(
                    deps,
                    env,
                    &liquidator_account_id,
                    &liquidatee_account_id,
                    debt_coin,
                    &request_coin_denom,
                ),
                LiquidateRequest::Lend(request_coin_denom) => liquidate_lend(
                    deps,
                    env,
                    &liquidator_account_id,
                    &liquidatee_account_id,
                    debt_coin,
                    &request_coin_denom,
                ),
                LiquidateRequest::Vault {
                    request_vault,
                    position_type,
                } => {
                    let request_vault = request_vault.check(deps.api)?;
                    liquidate_vault(
                        deps,
                        env,
                        &liquidator_account_id,
                        &liquidatee_account_id,
                        debt_coin,
                        request_vault,
                        position_type,
                    )
                }
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _: Env, msg: QueryMsg) -> ContractResult<Binary> {
    let res = match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
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
