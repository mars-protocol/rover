use cosmwasm_std::{Coin, Deps, DepsMut, Env, Response, Uint128, Uint256};

use mars_coin::Coin256;
use mars_rover::error::{ContractError, ContractResult};
use mars_rover::msg::execute::{ActionAmount, ActionCoin};
use mars_rover::traits::ToDenoms;

use crate::state::{COIN_BALANCES, ZAPPER};
use crate::utils::{
    assert_coin_is_whitelisted, assert_coins_are_whitelisted, decrement_coin_balance,
    update_balance_msg, update_balances_msgs,
};

pub fn provide_liquidity(
    deps: DepsMut,
    env: Env,
    account_id: &str,
    coins_in: Vec<ActionCoin>,
    lp_token_out: &str,
    minimum_receive: Uint128,
) -> ContractResult<Response> {
    assert_coin_is_whitelisted(deps.storage, lp_token_out)?;
    assert_coins_are_whitelisted(deps.storage, coins_in.to_denoms())?;

    // Decrement coin amounts in account for those sent to pool
    let mut updated_coins_in: Vec<Coin256> = Vec::with_capacity(coins_in.len());
    for coin_in in coins_in {
        let coin_balance = COIN_BALANCES.load(deps.storage, (account_id, &coin_in.denom))?;
        let new_amount = match coin_in.amount {
            ActionAmount::Exact(amt) => amt,
            ActionAmount::AccountBalance => coin_balance,
        };
        let updated_coin = Coin256 {
            denom: coin_in.denom,
            amount: new_amount,
        };
        decrement_coin_balance(deps.storage, account_id, &updated_coin)?;
        updated_coins_in.push(updated_coin);
    }

    // After zap is complete, update account's LP token balance
    let zapper = ZAPPER.load(deps.storage)?;
    let zap_msg = zapper.provide_liquidity_msg(
        // &updated_coins_in.try_into()?, TODO: <--- throws an error, need to debug
        &[],
        lp_token_out,
        minimum_receive,
    )?;
    let update_balance_msg = update_balance_msg(
        &deps.querier,
        &env.contract.address,
        account_id,
        lp_token_out,
    )?;

    Ok(Response::new()
        .add_message(zap_msg)
        .add_message(update_balance_msg)
        .add_attribute("action", "rover/credit-manager/provide_liquidity"))
}

pub fn withdraw_liquidity(
    deps: DepsMut,
    env: Env,
    account_id: &str,
    lp_token_action: &ActionCoin,
) -> ContractResult<Response> {
    assert_coin_is_whitelisted(deps.storage, &lp_token_action.denom)?;

    let lp_token = Coin256 {
        denom: lp_token_action.denom.clone(),
        amount: match lp_token_action.amount {
            ActionAmount::Exact(a) => a,
            ActionAmount::AccountBalance => COIN_BALANCES
                .may_load(deps.storage, (account_id, &lp_token_action.denom))?
                .unwrap_or(Uint256::zero()),
        },
    };

    if lp_token.amount.is_zero() {
        return Err(ContractError::NoAmount);
    }

    let zapper = ZAPPER.load(deps.storage)?;
    let coins_out =
        zapper.estimate_withdraw_liquidity(&deps.querier, &lp_token.clone().try_into()?)?;
    assert_coins_are_whitelisted(deps.storage, coins_out.to_denoms())?;

    decrement_coin_balance(deps.storage, account_id, &lp_token)?;

    // After unzap is complete, update account's coin balances
    let zap_msg = zapper.withdraw_liquidity_msg(&lp_token.try_into()?)?;
    let update_balances_msgs = update_balances_msgs(
        &deps.querier,
        &env.contract.address,
        account_id,
        coins_out.to_denoms(),
    )?;

    Ok(Response::new()
        .add_message(zap_msg)
        .add_messages(update_balances_msgs)
        .add_attribute("action", "rover/credit-manager/withdraw_liquidity"))
}

pub fn estimate_provide_liquidity(
    deps: Deps,
    lp_token_out: &str,
    coins_in: Vec<Coin>,
) -> ContractResult<Uint256> {
    let zapper = ZAPPER.load(deps.storage)?;
    let estimate = zapper.estimate_provide_liquidity(&deps.querier, lp_token_out, &coins_in)?;
    Ok(estimate)
}

pub fn estimate_withdraw_liquidity(deps: Deps, lp_token: Coin) -> ContractResult<Vec<Coin>> {
    let zapper = ZAPPER.load(deps.storage)?;
    let estimate = zapper.estimate_withdraw_liquidity(&deps.querier, &lp_token)?;
    Ok(estimate)
}
