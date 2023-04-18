use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Api, Coin, CosmosMsg, Decimal, QuerierWrapper, QueryRequest, StdResult,
    Uint128, WasmMsg, WasmQuery,
};
use mars_params::msg::QueryMsg;
use mars_params::types::{AssetParams, VaultConfig};

#[cw_serde]
pub struct MarsParamsBase<T>(T);

impl<T> MarsParamsBase<T> {
    pub fn new(address: T) -> MarsParamsBase<T> {
        MarsParamsBase(address)
    }

    pub fn address(&self) -> &T {
        &self.0
    }
}

pub type MarsParamsUnchecked = MarsParamsBase<String>;
pub type MarsParams = MarsParamsBase<Addr>;

impl From<MarsParams> for MarsParamsUnchecked {
    fn from(mars_params: MarsParams) -> Self {
        Self(mars_params.0.to_string())
    }
}

impl MarsParamsUnchecked {
    pub fn check(&self, api: &dyn Api) -> StdResult<MarsParams> {
        Ok(MarsParamsBase(api.addr_validate(self.address())?))
    }
}

impl MarsParams {
    pub fn query_asset_params(
        &self,
        querier: &QuerierWrapper,
        denom: &str,
    ) -> StdResult<AssetParams> {
        querier.query_wasm_smart(
            self.address().to_string(),
            &QueryMsg::AssetParams {
                denom: denom.to_string(),
            },
        )
    }

    pub fn query_vault_config(
        &self,
        querier: &QuerierWrapper,
        vault_address: &Addr,
    ) -> StdResult<VaultConfig> {
        querier.query_wasm_smart(
            self.address().to_string(),
            &QueryMsg::VaultConfig {
                address: vault_address.to_string(),
            },
        )
    }

    pub fn query_max_close_factor(
        &self,
        querier: &QuerierWrapper,
    ) -> StdResult<Decimal> {
        querier.query_wasm_smart(self.address().to_string(), &QueryMsg::MaxCloseFactor {})
    }
}
