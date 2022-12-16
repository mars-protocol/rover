use crate::ContractError;
use cosmwasm_std::{Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw_asset::Asset;
use cw_dex::traits::Pool;
use cw_dex::CwDexError;

pub trait LpPool {
    /// Returns the matching pool given a LP token.
    ///
    /// https://github.com/apollodao/cw-dex uses cargo feature flags for chain specific implementation.
    fn get_pool_for_lp_token(deps: Deps, lp_token_denom: &str)
        -> Result<Box<dyn Pool>, CwDexError>;

    fn execute_provide_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        lp_token_out: String,
        recipient: Option<String>,
        minimum_receive: Uint128,
    ) -> Result<Response, ContractError>;

    fn execute_callback_single_sided_join(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        asset: Asset,
        lp_token: String,
    ) -> Result<Response, ContractError>;

    fn query_estimate_provide_liquidity(
        deps: Deps,
        env: Env,
        lp_token_out: String,
        coins_in: Vec<Coin>,
    ) -> StdResult<Binary>;
}
