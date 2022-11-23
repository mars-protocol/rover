use cosmwasm_std::{
    to_binary, DepsMut, Empty, Env, Event, MessageInfo, QueryRequest, Response, WasmQuery,
};
use cw721::Cw721Execute;
use cw721_base::MintMsg;

use mars_rover::msg::query::HealthResponse;
use mars_rover::msg::QueryMsg::Health;

use crate::contract::Parent;
use crate::error::ContractError;
use crate::error::ContractError::{BaseError, BurnNotAllowed};
use crate::state::{CREDIT_MANAGER, MAX_VALUE_FOR_BURN, NEXT_ID, PENDING_OWNER};

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user: &str,
) -> Result<Response, ContractError> {
    let next_id = NEXT_ID.load(deps.storage)?;
    let mint_msg_override = MintMsg {
        token_id: next_id.to_string(),
        owner: user.to_string(),
        token_uri: None,
        extension: Empty {},
    };
    NEXT_ID.save(deps.storage, &(next_id + 1))?;

    Parent::default()
        .mint(deps, env, info, mint_msg_override)
        .map_err(Into::into)
}

/// Checks first to ensure the balance of debts and collateral does not exceed the config
/// set amount. This is to ensure accounts are not accidentally deleted.
pub fn burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let credit_manager = CREDIT_MANAGER.load(deps.storage)?;
    let response: HealthResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: credit_manager.into(),
        msg: to_binary(&Health {
            account_id: token_id.clone(),
        })?,
    }))?;

    let max_value_allowed = MAX_VALUE_FOR_BURN.load(deps.storage)?;
    let current_balances = response
        .total_debt_value
        .checked_add(response.total_collateral_value)?;
    if current_balances > max_value_allowed {
        return Err(BurnNotAllowed {
            current_balances,
            max_value_allowed,
        });
    }

    Parent::default()
        .burn(deps, env, info, token_id)
        .map_err(Into::into)
}

pub fn propose_new_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: &str,
) -> Result<Response, ContractError> {
    let proposed_owner_addr = deps.api.addr_validate(new_owner)?;
    let current_owner = Parent::default().minter.load(deps.storage)?;

    if info.sender != current_owner {
        return Err(BaseError(cw721_base::ContractError::Unauthorized {}));
    }

    PENDING_OWNER.save(deps.storage, &proposed_owner_addr)?;

    Ok(Response::new().add_attribute("action", "rover/account_nft/propose_new_owner"))
}

pub fn accept_ownership(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let pending_owner = PENDING_OWNER.load(deps.storage)?;
    let previous_owner = Parent::default().minter.load(deps.storage)?;

    if info.sender != pending_owner {
        return Err(BaseError(cw721_base::ContractError::Unauthorized {}));
    }

    Parent::default()
        .minter
        .save(deps.storage, &pending_owner)?;

    PENDING_OWNER.remove(deps.storage);

    let event = Event::new("rover/account_nft/accept_ownership")
        .add_attribute("previous_owner", previous_owner)
        .add_attribute("new_owner", pending_owner);
    Ok(Response::new().add_event(event))
}
