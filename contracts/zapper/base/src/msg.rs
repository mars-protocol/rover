use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Env, StdResult, Uint128, WasmMsg};
use cw_asset::Asset;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    ProvideLiquidity {
        lp_token_out: String,
        recipient: Option<String>,
        minimum_receive: Uint128,
    },
    WithdrawLiquidity {
        recipient: Option<String>,
    },
    Callback(CallbackMsg),
}

#[cw_serde]
pub enum CallbackMsg {
    SingleSidedJoin {
        asset: Asset,
        lp_token: String,
    },
    ReturnLpTokens {
        balance_before: Asset,
        recipient: Addr,
        minimum_receive: Uint128,
    },
}

impl CallbackMsg {
    pub fn into_cosmos_msg(self, env: &Env) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_binary(&ExecuteMsg::Callback(self))?,
            funds: vec![],
        }))
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Uint128)]
    EstimateProvideLiquidity {
        lp_token_out: String,
        coins_in: Vec<Coin>,
    },
    #[returns(Vec<Coin>)]
    EstimateWithdrawLiquidity { coin_in: Coin },
}
