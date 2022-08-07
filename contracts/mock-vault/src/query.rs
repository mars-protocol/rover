use cosmwasm_std::{Coin, Deps, Order, StdResult, Storage, Uint128};

use rover::msg::vault::{UnlockingTokens, VaultInfo};

use crate::state::{ASSETS, LOCKUP_TIME, LP_TOKEN_DENOM, TOTAL_VAULT_SHARES, UNLOCKING_TOKENS};

pub fn query_assets_for_shares(storage: &dyn Storage, shares: Uint128) -> StdResult<Vec<Coin>> {
    let total_shares_opt = TOTAL_VAULT_SHARES.may_load(storage)?;
    match total_shares_opt {
        None => Ok(vec![]),
        Some(total_vault_shares) => {
            let all_vault_assets = get_all_vault_assets(storage)?;
            let assets_for_shares = all_vault_assets
                .iter()
                .map(|asset| Coin {
                    denom: asset.clone().denom,
                    amount: asset.amount.multiply_ratio(shares, total_vault_shares),
                })
                .collect::<Vec<Coin>>();
            Ok(assets_for_shares)
        }
    }
}

pub fn query_vault_info(deps: Deps) -> StdResult<VaultInfo> {
    Ok(VaultInfo {
        assets: get_all_vault_assets(deps.storage)?,
        lockup: LOCKUP_TIME.load(deps.storage)?,
        token_denom: LP_TOKEN_DENOM.load(deps.storage)?,
    })
}

pub fn query_unlocking_positions(deps: Deps, addr: String) -> StdResult<Vec<UnlockingTokens>> {
    let addr = deps.api.addr_validate(addr.as_str())?;
    let res = UNLOCKING_TOKENS.load(deps.storage, addr)?;
    Ok(res)
}

pub fn get_all_vault_assets(storage: &dyn Storage) -> StdResult<Vec<Coin>> {
    Ok(ASSETS
        .range(storage, None, None, Order::Ascending)
        .collect::<StdResult<Vec<_>>>()?
        .iter()
        .map(|(denom, amount)| Coin {
            denom: denom.clone(),
            amount: *amount,
        })
        .collect::<Vec<_>>())
}
