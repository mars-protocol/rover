use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, DepsMut, Response, Uint128};

use crate::state::COIN_BALANCES;
use rover::error::ContractResult;

use crate::utils::assert_coin_is_whitelisted;

pub fn withdraw(
    deps: DepsMut,
    token_id: &str,
    coin: Coin,
    recipient: Addr,
) -> ContractResult<Response> {
    assert_coin_is_whitelisted(deps.storage, &coin)?;

    if coin.amount.is_zero() {
        return Ok(Response::new());
    }

    // decrement the token's asset amount
    let path = COIN_BALANCES.key((token_id, &coin.denom));
    let value_opt = path.may_load(deps.storage)?;
    let new_value = value_opt
        .unwrap_or_else(Uint128::zero)
        .checked_sub(coin.amount)?;
    if new_value.is_zero() {
        path.remove(deps.storage);
    } else {
        path.save(deps.storage, &new_value)?;
    }

    // send coin to recipient
    let transfer_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient.to_string(),
        amount: vec![coin.clone()],
    });

    Ok(Response::new()
        .add_message(transfer_msg)
        .add_attribute("action", "rover/credit_manager/callback/withdraw")
        .add_attribute("withdrawn", coin.to_string()))
}
