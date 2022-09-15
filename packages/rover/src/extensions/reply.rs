use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Reply, StdError, StdResult, SubMsgResult, Uint128};

#[cw_serde]
pub struct AssetTransferMsg {
    pub recipient: String,
    pub sender: String,
    pub amount: Vec<Coin>,
}

#[cw_serde]
pub struct UnlockEvent {
    pub id: Uint128,
    pub vault_addr: String,
}

pub trait AttrParse {
    fn parse_unlock_event(self) -> StdResult<UnlockEvent>;
}

impl AttrParse for Reply {
    fn parse_unlock_event(self) -> StdResult<UnlockEvent> {
        match self.result {
            SubMsgResult::Err(err) => Err(StdError::generic_err(err)),
            SubMsgResult::Ok(response) => {
                let unlock_event = response
                    .events
                    .iter()
                    .find(|event| event.ty == "wasm-unlocking_position_created")
                    .ok_or_else(|| StdError::generic_err("No unlock event"))?;

                let id = &unlock_event
                    .attributes
                    .iter()
                    .find(|x| x.key == "id")
                    .ok_or_else(|| StdError::generic_err("No id attribute"))?
                    .value;
                // parse_instantiate_response_data
                // https://github.com/CosmWasm/wasmd/blob/main/EVENTS.md#standard-events-in-xwasm
                let contract_addr = &unlock_event
                    .attributes
                    .iter()
                    .find(|x| x.key == "_contract_addr")
                    .ok_or_else(|| StdError::generic_err("No contract attribute"))?
                    .value;

                Ok(UnlockEvent {
                    id: Uint128::from_str(id.as_str())?,
                    vault_addr: contract_addr.to_string(),
                })
            }
        }
    }
}
