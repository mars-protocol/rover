use cosmwasm_std::{
    to_binary, BalanceResponse, BankQuery, Coin, CosmosMsg, Deps, DepsMut, Env, QueryRequest,
    Response, StdResult, Uint128, WasmMsg,
};

use rover::adapters::Vault;
use rover::error::ContractResult;
use rover::msg::execute::CallbackMsg;
use rover::msg::vault::ExecuteMsg;
use rover::msg::ExecuteMsg as RoverExecuteMsg;
use rover::NftTokenId;

use crate::utils::{decrement_coin_balance, increment_coin_balance};
use crate::vault::utils::{assert_vault_is_whitelisted, decrement_vault_position};

pub fn withdraw_from_vault(
    deps: DepsMut,
    env: Env,
    token_id: NftTokenId,
    vault: Vault,
    amount: Uint128,
    force: bool,
) -> ContractResult<Response> {
    assert_vault_is_whitelisted(deps.storage, &vault)?;

    decrement_vault_position(deps.storage, token_id, &vault, amount, force)?;

    // Sends vault coins to vault in exchange for underlying assets
    let vault_info = vault.query_vault_info(&deps.querier)?;
    let withdraw_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: vault.address().to_string(),
        funds: vec![Coin {
            denom: vault_info.token_denom,
            amount,
        }],
        msg: to_binary(
            &(if force {
                ExecuteMsg::ForceWithdraw {}
            } else {
                ExecuteMsg::Withdraw {}
            }),
        )?,
    });

    // Updates coin balances for account after a vault withdraw has taken place
    let previous_balances = query_balances(deps.as_ref(), &env, &vault_info.coins)?;
    let update_coin_balance_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        funds: vec![],
        msg: to_binary(&RoverExecuteMsg::Callback(
            CallbackMsg::UpdateCoinBalances {
                token_id: token_id.to_string(),
                previous_balances,
            },
        ))?,
    });

    Ok(Response::new()
        .add_message(withdraw_msg)
        .add_message(update_coin_balance_msg)
        .add_attribute("action", "rover/credit_manager/vault/withdraw"))
}

fn query_balance(deps: Deps, env: &Env, coin: &Coin) -> StdResult<Coin> {
    let res: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: env.contract.address.to_string(),
        denom: coin.denom.clone(),
    }))?;
    Ok(Coin {
        denom: coin.denom.clone(),
        amount: res.amount.amount,
    })
}

fn query_balances(deps: Deps, env: &Env, coins: &[Coin]) -> StdResult<Vec<Coin>> {
    coins
        .iter()
        .map(|coin| query_balance(deps, env, coin))
        .collect()
}

pub fn update_coin_balances(
    deps: DepsMut,
    env: Env,
    token_id: NftTokenId,
    previous_balances: &[Coin],
) -> ContractResult<Response> {
    let mut response = Response::new();

    for prev in previous_balances {
        let curr = query_balance(deps.as_ref(), &env, prev)?;
        if prev.amount > curr.amount {
            let new_amount = prev.amount.checked_sub(curr.amount)?;
            decrement_coin_balance(
                deps.storage,
                token_id,
                &Coin {
                    denom: curr.denom.clone(),
                    amount: new_amount,
                },
            )?;
            response = response
                .clone()
                .add_attribute("denom", curr.denom.clone())
                .add_attribute("decremented", new_amount);
        } else {
            let new_amount = curr.amount.checked_sub(prev.amount)?;
            increment_coin_balance(
                deps.storage,
                token_id,
                &Coin {
                    denom: curr.denom.clone(),
                    amount: new_amount,
                },
            )?;
            response = response
                .clone()
                .add_attribute("denom", curr.denom.clone())
                .add_attribute("incremented", new_amount);
        }
    }

    Ok(response.add_attribute("action", "rover/credit_manager/vault/update_coin_balance"))
}
