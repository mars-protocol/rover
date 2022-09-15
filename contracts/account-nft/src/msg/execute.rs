use std::convert::TryInto;

use cosmwasm_std::{Binary, Empty, StdError};
use cw721::Expiration;
use cw721_base::{ContractError, ExecuteMsg as ParentExecuteMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    //--------------------------------------------------------------------------------------------------
    // Extended and overridden messages
    //--------------------------------------------------------------------------------------------------
    /// Due to some chains being permissioned via governance, we must instantiate this contract first
    /// and give ownership access to Rover contract with this action after both are independently deployed.
    ProposeNewOwner { new_owner: String },

    /// Accept the proposed ownership transfer
    AcceptOwnership {},

    /// Mint a new NFT to the specified user; can only be called by the contract minter
    Mint { user: String },

    //--------------------------------------------------------------------------------------------------
    // Base cw721 messages
    //--------------------------------------------------------------------------------------------------
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft { recipient: String, token_id: String },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke { spender: String, token_id: String },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    /// Burn an NFT the sender has access to
    Burn { token_id: String },
}

impl TryInto<ParentExecuteMsg<Empty>> for ExecuteMsg {
    type Error = ContractError;

    fn try_into(self) -> Result<ParentExecuteMsg<Empty>, Self::Error> {
        match self {
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Ok(ParentExecuteMsg::TransferNft {
                recipient,
                token_id,
            }),
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => Ok(ParentExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            }),
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => Ok(ParentExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            }),
            ExecuteMsg::Revoke { spender, token_id } => {
                Ok(ParentExecuteMsg::Revoke { spender, token_id })
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                Ok(ParentExecuteMsg::ApproveAll { operator, expires })
            }
            ExecuteMsg::RevokeAll { operator } => Ok(ParentExecuteMsg::RevokeAll { operator }),
            ExecuteMsg::Burn { token_id } => Ok(ParentExecuteMsg::Burn { token_id }),
            _ => Err(StdError::generic_err(
                "Attempting to convert to a non-cw721 compatible message",
            )
            .into()),
        }
    }
}
