use cosmwasm_std::{Deps, Uint128};
use cw_asset::{Asset, AssetInfo, AssetList};
use cw_dex::traits::Pool;
use cw_dex::CwDexError;

pub trait LpPool {
    /// Returns the matching pool given a LP token.
    ///
    /// https://github.com/apollodao/cw-dex uses cargo feature flags for chain specific implementation.
    fn get_pool_for_lp_token(deps: Deps, lp_token: &AssetInfo)
        -> Result<Box<dyn Pool>, CwDexError>;

    fn simulate_noswap_join(
        deps: Deps,
        lp_token: &AssetInfo,
        assets: &AssetList,
    ) -> Result<(Uint128, AssetList), CwDexError>;

    fn simulate_single_sided_join(
        deps: Deps,
        lp_token: &AssetInfo,
        asset: &Asset,
    ) -> Result<Uint128, CwDexError>;
}
