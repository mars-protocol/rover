use std::convert::TryFrom;

use cosmwasm_std::{Coin, Reply, StdError, StdResult, SubMsgResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use regex::Regex;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CoinTransferMsg {
    pub recipient: String,
    pub sender: String,
    pub coins: Vec<Coin>,
}

pub trait AttrParse {
    fn parse_transfer_msg(self) -> StdResult<CoinTransferMsg>;
}

impl AttrParse for Reply {
    fn parse_transfer_msg(self) -> StdResult<CoinTransferMsg> {
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

                Ok(CoinTransferMsg {
                    recipient: recipient.to_string(),
                    sender: sender.to_string(),
                    coins: amount.clone(),
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
