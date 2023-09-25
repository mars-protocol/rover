use cosmwasm_std::{
    to_binary, DepsMut, Empty, QueryRequest, Response, StdError, Storage, WasmQuery,
};
use cw2::set_contract_version;
use cw721::Cw721Query;
use mars_account_nft_types::{msg::ClearEmptyAccounts, nft_config::NftConfig};
use mars_red_bank_types::oracle::ActionKind;
use mars_rover_health_types::{AccountKind, HealthValuesResponse, QueryMsg::HealthValues};

use crate::{
    contract::{Parent, CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractError::{self, HealthContractNotSet},
    state::{ClearingMarker, CONFIG, MIGRATION_CLEARING_MARKER},
};

const FROM_VERSION: &str = "1.0.0";

pub mod v1_state {
    use cw_storage_plus::Item;
    use mars_rover_old::adapters::account_nft::NftConfig;

    pub const CONFIG: Item<NftConfig> = Item::new("config");
}

pub fn migrate(deps: DepsMut) -> Result<Response, ContractError> {
    // make sure we're migrating the correct contract and from the correct version
    cw2::assert_contract_version(
        deps.as_ref().storage,
        &format!("crates.io:{CONTRACT_NAME}"),
        FROM_VERSION,
    )?;

    // CONFIG updated, re-initializing
    let old_config_state = v1_state::CONFIG.load(deps.storage)?;
    v1_state::CONFIG.remove(deps.storage);
    CONFIG.save(
        deps.storage,
        &NftConfig {
            // health_contract_addr and credit_manager_contract_addr can be updated via update_config
            max_value_for_burn: old_config_state.max_value_for_burn,
            health_contract_addr: None,
            credit_manager_contract_addr: None,
        },
    )?;

    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;

    Ok(cw721_base::upgrades::v0_17::migrate::<Empty, Empty, Empty, Empty>(deps)?)
}

pub fn clear_empty_accounts(
    mut deps: DepsMut,
    msg: ClearEmptyAccounts,
) -> Result<Response, ContractError> {
    let clearing_marker_opt = MIGRATION_CLEARING_MARKER.may_load(deps.storage)?;
    let start_after = match clearing_marker_opt {
        Some(ClearingMarker::Finished) => {
            return Err(StdError::generic_err(
                "Migration completed. All empty accounts already burned.",
            )
            .into())
        }
        Some(ClearingMarker::StartAfter(start_after)) => Some(start_after),
        None => None,
    };

    let res = Parent::default().all_tokens(deps.as_ref(), start_after, msg.limit)?;

    if let Some(last_token) = res.tokens.last() {
        // Save last token for next iteration
        MIGRATION_CLEARING_MARKER
            .save(deps.storage, &ClearingMarker::StartAfter(last_token.clone()))?;
    } else {
        // No more tokens. Migration finished
        MIGRATION_CLEARING_MARKER.save(deps.storage, &ClearingMarker::Finished)?;
    }

    for token in res.tokens.into_iter() {
        burn_empty_account(deps.branch(), token)?;
    }

    Ok(Response::new().add_attribute("action", "burn_empty_accounts"))
}

/// A few checks to ensure accounts are not accidentally deleted:
/// - Cannot burn if debt balance
/// - Cannot burn if collateral balance
fn burn_empty_account(deps: DepsMut, token_id: String) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let Some(health_contract_addr) = config.health_contract_addr else {
        return Err(HealthContractNotSet);
    };

    let response: HealthValuesResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: health_contract_addr.into(),
            msg: to_binary(&HealthValues {
                account_id: token_id.clone(),
                kind: AccountKind::Default,
                action: ActionKind::Default,
            })?,
        }))?;

    // Burn only empty accounts
    if response.total_debt_value.is_zero() && response.total_collateral_value.is_zero() {
        burn_without_owner_check(deps.storage, token_id)?;
    }

    Ok(())
}

fn burn_without_owner_check(
    storage: &mut dyn Storage,
    token_id: String,
) -> Result<(), ContractError> {
    let parent = Parent::default();

    // Original function has additional check for token owner:
    // let token = parnet.tokens.load(deps.storage, &token_id)?;
    // parnet.check_can_send(deps.as_ref(), &env, &info, &token)?;

    Parent::default().tokens.remove(storage, &token_id)?;
    parent.decrement_tokens(storage)?;

    Ok(())
}
