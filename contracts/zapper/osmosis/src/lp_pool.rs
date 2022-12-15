use cosmwasm_std::{Deps, Uint128};
use cw_asset::{Asset, AssetInfo, AssetList};
use cw_dex::traits::Pool;
use cw_dex::CwDexError;
use mars_zapper_base::LpPool;

pub struct OsmosisPool {}

impl LpPool for OsmosisPool {
    fn get_pool_for_lp_token(
        deps: Deps,
        lp_token: &AssetInfo,
    ) -> Result<Box<dyn Pool>, CwDexError> {
        cw_dex::Pool::get_pool_for_lp_token(deps, lp_token).map(|p| p.as_trait())
    }

    fn simulate_noswap_join(
        deps: Deps,
        lp_token: &AssetInfo,
        assets: &AssetList,
    ) -> Result<(Uint128, AssetList), CwDexError> {
        let pool = cw_dex::Pool::get_pool_for_lp_token(deps, lp_token)?;
        match pool {
            cw_dex::Pool::Osmosis(osmo_pool) => {
                let res = osmo_pool.simulate_noswap_join(&deps.querier, assets)?;
                Ok(res)
            }
            _ => unimplemented!(),
        }
    }

    fn simulate_single_sided_join(
        deps: Deps,
        lp_token: &AssetInfo,
        asset: &Asset,
    ) -> Result<Uint128, CwDexError> {
        let pool = cw_dex::Pool::get_pool_for_lp_token(deps, lp_token)?;
        match pool {
            cw_dex::Pool::Osmosis(osmo_pool) => {
                let res = osmo_pool.simulate_single_sided_join(&deps.querier, asset)?;
                Ok(res)
            }
            _ => unimplemented!(),
        }
    }
}
