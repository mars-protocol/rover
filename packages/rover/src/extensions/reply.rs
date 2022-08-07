use std::convert::TryFrom;
use std::str::FromStr;

use cosmwasm_std::{Coin, Reply, StdError, StdResult, SubMsgResult, Uint128};
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AssetTransferMsg {
    pub recipient: String,
    pub sender: String,
    pub amount: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct UnlockEvent {
    pub id: Uint128,
    pub vault_addr: String,
}

pub trait AttrParse {
    fn parse_transfer_msg(self) -> StdResult<AssetTransferMsg>;
    fn parse_unlock_event(self) -> StdResult<UnlockEvent>;
}

impl AttrParse for Reply {
    fn parse_transfer_msg(self) -> StdResult<AssetTransferMsg> {
        match self.result {
            SubMsgResult::Err(err) => Err(StdError::generic_err(err)),
            SubMsgResult::Ok(response) => {
                let transfer_event = response
                    .events
                    .iter()
                    .find(|event| event.ty == "transfer")
                    .ok_or_else(|| StdError::generic_err("No transfer event"))?;

                let recipient = &transfer_event
                    .attributes
                    .iter()
                    .find(|x| x.key == "recipient")
                    .ok_or_else(|| StdError::generic_err("No recipient attribute"))?
                    .value;

                let sender = &transfer_event
                    .attributes
                    .iter()
                    .find(|x| x.key == "sender")
                    .ok_or_else(|| StdError::generic_err("No sender attribute"))?
                    .value;

                //  { key: "amount", amount: "23uatom,120uosmo" }
                let amount = &transfer_event
                    .attributes
                    .iter()
                    .find(|x| x.key == "amount")
                    .ok_or_else(|| StdError::generic_err("No amount attribute"))?
                    .value
                    .split(',')
                    .map(str_to_coin)
                    .collect::<StdResult<Vec<Coin>>>()?;

                Ok(AssetTransferMsg {
                    recipient: recipient.to_string(),
                    sender: sender.to_string(),
                    amount: amount.clone(),
                })
            }
        }
    }

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

/// Parses string in the format <amount><denom> (e.g. 86uatom) to `Coin`
fn str_to_coin(coin_str: &str) -> StdResult<Coin> {
    let word_re =
        Regex::new(r"\D+").map_err(|_| StdError::generic_err("Could not parse letters"))?;

    let num_re =
        Regex::new(r"\d+").map_err(|_| StdError::generic_err("Could not parse numbers"))?;

    let amount_str = num_re
        .find(coin_str)
        .ok_or_else(|| StdError::generic_err("No amount present"))?
        .as_str();

    let denom = word_re
        .find(coin_str)
        .ok_or_else(|| StdError::generic_err("No denom present"))?
        .as_str();

    Ok(Coin {
        denom: denom.to_string(),
        amount: Uint128::try_from(amount_str)?,
    })
}
