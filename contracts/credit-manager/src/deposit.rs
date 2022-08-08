use cosmwasm_std::{Coin, Response, Storage, Uint128};

use rover::coins::Coins;
use rover::error::{ContractError, ContractResult};

use crate::state::COIN_BALANCES;
use crate::utils::assert_coin_is_whitelisted;

pub fn deposit(
    storage: &mut dyn Storage,
    response: Response,
    nft_token_id: &str,
    coin: &Coin,
    received_coins: &mut Coins,
) -> ContractResult<Response> {
    assert_coin_is_whitelisted(storage, coin)?;

    if coin.amount.is_zero() {
        return Ok(response);
    }

    assert_sent_fund(coin, received_coins)?;

    received_coins.deduct(coin)?;

    // increase the token's asset amount
    increment_position(storage, nft_token_id, coin)?;

    Ok(response
        .add_attribute("action", "rover/credit_manager/callback/deposit")
        .add_attribute("deposit_received", coin.to_string()))
}

/// Assert that fund of exactly the same type and amount was sent along with a message
fn assert_sent_fund(expected: &Coin, received_coins: &Coins) -> ContractResult<()> {
    let received = received_coins
        .amount(&expected.denom)
        .unwrap_or_else(Uint128::zero);

    if received != expected.amount {
        return Err(ContractError::FundsMismatch {
            expected: expected.amount,
            received,
        });
    }

    Ok(())
}

fn increment_position(
    storage: &mut dyn Storage,
    token_id: &str,
    coin: &Coin,
) -> ContractResult<()> {
    COIN_BALANCES.update(
        storage,
        (token_id, &coin.denom),
        |value_opt| -> ContractResult<_> {
            value_opt
                .unwrap_or_else(Uint128::zero)
                .checked_add(coin.amount)
                .map_err(ContractError::Overflow)
        },
    )?;
    Ok(())
}
