use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError};
use mars_rover::{
    adapters::vault::{UnlockingChange, VaultPositionUpdate::Unlocking, VaultUnchecked},
    error::{ContractError::Std, ContractResult},
};

use crate::state::{OWNER, VAULT_POSITIONS};

pub fn update_lockup_id(
    deps: DepsMut,
    info: MessageInfo,
    account_id: &str,
    unchecked: VaultUnchecked,
    current_lockup_id: u64,
    new_lockup_id: u64,
) -> ContractResult<Response> {
    OWNER.assert_owner(deps.storage, &info.sender)?;

    let vault = unchecked.check(deps.api)?;
    let mut position = VAULT_POSITIONS.load(deps.storage, (account_id, vault.address.clone()))?;

    let Some(mut unlocking_position) = position.get_unlocking_position(current_lockup_id) else {
        return Err(Std(StdError::generic_err(format!("Lockup id: {current_lockup_id} not found for account id"))));
    };

    // Remove old position
    position.update(Unlocking(UnlockingChange::Decrement {
        id: unlocking_position.id,
        amount: unlocking_position.coin.amount,
    }))?;

    // Update to new id
    unlocking_position.id = new_lockup_id;

    // Add it back with updated id
    position.update(Unlocking(UnlockingChange::Add(unlocking_position)))?;

    VAULT_POSITIONS.save(deps.storage, (account_id, vault.address), &position)?;

    Ok(Response::new()
        .add_attribute("account_id", account_id)
        .add_attribute("vault", unchecked.address)
        .add_attribute("old_lockup_id", current_lockup_id.to_string())
        .add_attribute("new_lockup_id", new_lockup_id.to_string()))
}
