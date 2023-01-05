use cosmwasm_std::{
    Addr, BalanceResponse, BankQuery, DepsMut, Env, QuerierWrapper, QueryRequest, Response,
    StdResult,
};

use mars_coin::Coin256;
use mars_rover::error::ContractResult;

use crate::utils::{decrement_coin_balance, increment_coin_balance};

pub fn query_balance(querier: &QuerierWrapper, addr: &Addr, denom: &str) -> StdResult<Coin256> {
    let res: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: addr.to_string(),
        denom: denom.to_string(),
    }))?;
    Ok(Coin256 {
        denom: denom.to_string(),
        amount: res.amount.amount.into(),
    })
}

pub fn update_coin_balance(
    deps: DepsMut,
    env: Env,
    account_id: &str,
    prev: &Coin256,
) -> ContractResult<Response> {
    let curr = query_balance(&deps.querier, &env.contract.address, &prev.denom)?;
    if prev.amount > curr.amount {
        let amount_to_reduce = prev.amount.checked_sub(curr.amount)?;
        decrement_coin_balance(
            deps.storage,
            account_id,
            &Coin256 {
                denom: curr.denom.clone(),
                amount: amount_to_reduce,
            },
        )?;
        Ok(Response::new()
            .add_attribute("action", "rover/credit-manager/update_coin_balance")
            .add_attribute("denom", curr.denom)
            .add_attribute("decremented", amount_to_reduce))
    } else {
        let amount_to_increment = curr.amount.checked_sub(prev.amount)?;
        increment_coin_balance(
            deps.storage,
            account_id,
            &Coin256 {
                denom: curr.denom.clone(),
                amount: amount_to_increment,
            },
        )?;
        Ok(Response::new()
            .add_attribute("action", "rover/credit-manager/update_coin_balance")
            .add_attribute("denom", curr.denom)
            .add_attribute("incremented", amount_to_increment))
    }
}
