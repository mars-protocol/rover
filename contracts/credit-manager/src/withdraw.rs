use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Response, Uint128};
use mars_rover::{
    error::{ContractError, ContractResult},
    msg::execute::ActionCoin,
};

use crate::{state::COIN_BALANCES, utils::decrement_coin_balance};

pub fn withdraw(
    deps: DepsMut,
    account_id: &str,
    coin: &ActionCoin,
    recipient: Addr,
) -> ContractResult<Response> {
    let amount_to_withdraw = Coin {
        denom: coin.denom.to_string(),
        amount: get_withdraw_amount(deps.as_ref(), account_id, coin)?,
    };

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
        .add_attribute("coin_withdrawn", format!("{}{}",&coin.denom, amount_to_withdraw.amount)))
}

/// Queries
/// Also asserts the amount is greater than zero.
fn get_withdraw_amount(deps: Deps, account_id: &str, coin: &ActionCoin) -> ContractResult<Uint128> {
    if let Some(amount) = coin.amount.value() {
        return Ok(amount);
    }

    let Some(amount) = COIN_BALANCES.may_load(deps.storage, (account_id, &coin.denom))? else {
        return Err(ContractError::NoAmount);
    };

    Ok(amount)
}
