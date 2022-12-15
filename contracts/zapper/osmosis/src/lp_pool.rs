use cosmwasm_std::Deps;
use cw_asset::AssetInfo;
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
}
