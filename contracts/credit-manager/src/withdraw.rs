use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Response, StdResult};
use mars_rover::{
    error::{ContractError, ContractResult},
    msg::execute::{ActionAmount, ActionCoin},
};

use crate::{
    state::COIN_BALANCES, update_coin_balances::query_balance, utils::decrement_coin_balance,
};

pub fn withdraw(
    deps: DepsMut,
    account_id: &str,
    coin: &ActionCoin,
    recipient: Addr,
) -> ContractResult<Response> {
    let amount_to_withdraw = get_withdraw_amount(deps.as_ref(), account_id, coin)?;

    decrement_coin_balance(deps.storage, account_id, &amount_to_withdraw)?;

    // send coin to recipient
    let transfer_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient.to_string(),
        amount: vec![amount_to_withdraw.clone()],
    });

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "callback/withdraw")
        .add_attribute("account_id", account_id)
        .add_attribute("coin_withdrawn", amount_to_withdraw.to_string()))
}

/// Checks if Exact or Account Balance is passed through Action Coin
/// Also asserts the amount is greater than zero.
fn get_withdraw_amount(deps: Deps, account_id: &str, coin: &ActionCoin) -> ContractResult<Coin> {
    let amount = match coin.amount {
        ActionAmount::Exact(amount) => amount,
        ActionAmount::AccountBalance => {
            COIN_BALANCES.may_load(deps.storage, (account_id, &coin.denom))?.unwrap_or_default()
        }
    };

    if amount.is_zero() {
        return Err(ContractError::NoAmount);
    }

    let coin = Coin {
        denom: coin.denom.clone(),
        amount,
    };

    Ok(coin)
}

pub fn send(
    deps: DepsMut,
    rover_addr: &Addr,
    account_id: &str,
    recipient: Addr,
    previous_balances: Vec<Coin>,
) -> ContractResult<Response> {
    let coins = previous_balances
        .into_iter()
        .map(|coin| {
            let current_balance = query_balance(&deps.querier, rover_addr, &coin.denom)?;
            let amount_to_withdraw = current_balance.amount.checked_sub(coin.amount)?;
            Ok(Coin {
                denom: coin.denom,
                amount: amount_to_withdraw,
            })
        })
        .collect::<StdResult<Vec<_>>>()?;

    // send coin to recipient
    let transfer_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient.to_string(),
        amount: coins,
    });

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "callback/send")
        .add_attribute("account_id", account_id))
}
