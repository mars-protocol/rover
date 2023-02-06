use std::collections::HashMap;

use cosmwasm_std::{Deps, StdResult};
use mars_rover_health_computer::{DenomsData, HealthComputer, VaultsData};
use mars_rover_health_types::{HealthError::CreditManagerNotSet, HealthResponse, HealthResult};

use crate::{querier::HealthQuerier, state::CREDIT_MANAGER};

/// Uses `mars-rover-health-computer` which is a data agnostic package given
/// it's compiled to .wasm and shared with the frontend.
/// This function queries all necessary data to pass to `HealthComputer`.
pub fn compute_health(deps: Deps, account_id: &str) -> HealthResult<HealthResponse> {
    let credit_manager_addr =
        CREDIT_MANAGER.may_load(deps.storage)?.ok_or(CreditManagerNotSet {})?;
    let querier = HealthQuerier::new(&deps.querier, &credit_manager_addr);

    let positions = querier.query_positions(account_id)?;

    // Get the denoms that need prices + markets
    let deposit_denoms =
        positions.deposits.clone().into_iter().map(|d| d.denom).collect::<Vec<_>>();
    let debt_denoms = positions.debts.clone().into_iter().map(|d| d.denom).collect::<Vec<_>>();

    let vault_infos = positions
        .vaults
        .clone()
        .into_iter()
        .map(|v| {
            let info = v.vault.query_info(&deps.querier)?;
            Ok((v.vault.address, info))
        })
        .collect::<StdResult<HashMap<_, _>>>()?;

    let vault_base_token_denoms =
        vault_infos.values().map(|v| v.base_token.clone()).collect::<Vec<_>>();

    // Collect prices + markets
    let (oracle, red_bank) = querier.query_deps()?;
    let mut denoms_data: DenomsData = Default::default();
    deposit_denoms.into_iter().chain(debt_denoms).chain(vault_base_token_denoms).try_for_each(
        |denom| -> StdResult<()> {
            let price = oracle.query_price(&deps.querier, &denom)?.price;
            denoms_data.prices.insert(denom.clone(), price);
            let market = red_bank.query_market(&deps.querier, &denom)?;
            denoms_data.markets.insert(denom, market);
            Ok(())
        },
    )?;

    // Collect all vault data
    let mut vaults_data: VaultsData = Default::default();

    positions.vaults.clone().into_iter().try_for_each(|v| -> HealthResult<()> {
        let vault_coin_value = v.query_values(&deps.querier, &oracle)?;
        vaults_data.vault_values.insert(v.vault.address.clone(), vault_coin_value);

        let config = querier.query_vault_config(&v.vault)?;
        vaults_data.vault_configs.insert(v.vault.address, config);
        Ok(())
    })?;

    let allowed_coins = querier.query_allowed_coins()?;

    let computer = HealthComputer {
        positions,
        denoms_data,
        vaults_data,
        allowed_coins,
    };

    Ok(computer.compute_health()?.into())
}
