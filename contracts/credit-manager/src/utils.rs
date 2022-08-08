use crate::state::ALLOWED_COINS;
use cosmwasm_std::{Coin, Storage};
use rover::error::{ContractError, ContractResult};

pub fn assert_coin_is_whitelisted(storage: &mut dyn Storage, coin: &Coin) -> ContractResult<()> {
    let is_whitelisted = ALLOWED_COINS.has(storage, &coin.denom);
    if !is_whitelisted {
        return Err(ContractError::NotWhitelisted(coin.denom.clone()));
    }
    Ok(())
}
