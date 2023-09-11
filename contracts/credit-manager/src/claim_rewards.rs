use cosmwasm_std::{Addr, DepsMut, Env, Response};
use mars_rover::{
    error::{ContractError, ContractResult},
    msg::execute::ChangeExpected,
    traits::Denoms,
};
use mars_rover_health_types::AccountKind;

use crate::{
    state::INCENTIVES,
    utils::{get_account_kind, send_balances_msgs, update_balances_msgs},
};

pub fn claim_rewards(
    deps: DepsMut,
    env: Env,
    account_id: &str,
    recipient: Addr,
) -> ContractResult<Response> {
    let incentives = INCENTIVES.load(deps.storage)?;

    let unclaimed_rewards = incentives.query_unclaimed_rewards(&deps.querier, account_id)?;
    if unclaimed_rewards.is_empty() {
        return Err(ContractError::NoAmount);
    }

    // For HLS accounts there are special requirements enforced for this account type.
    // assert_hls_rules only allows assets with HLS params set in the params contract
    // and where the collateral is whitelisted.
    // We withdraw all claimed rewards for HLS accounts to the recipient address.
    let kind = get_account_kind(deps.storage, account_id)?;
    let msgs = match kind {
        AccountKind::Default => update_balances_msgs(
            &deps.querier,
            &env.contract.address,
            account_id,
            unclaimed_rewards.to_denoms(),
            ChangeExpected::Increase,
        )?,
        AccountKind::HighLeveredStrategy => {
            let msg = send_balances_msgs(
                &deps.querier,
                &env.contract.address,
                account_id,
                recipient,
                unclaimed_rewards.to_denoms(),
            )?;
            vec![msg]
        }
    };

    Ok(Response::new()
        .add_message(incentives.claim_rewards_msg(account_id)?)
        .add_messages(msgs)
        .add_attribute("action", "claim_rewards")
        .add_attribute("account_id", account_id))
}
