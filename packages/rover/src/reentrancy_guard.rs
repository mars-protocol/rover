use std::fmt::Debug;

use cosmwasm_schema::{cw_serde, schemars::JsonSchema};
use cosmwasm_std::{Response, StdResult, Storage};
use cw_storage_plus::Item;

use crate::error::{ContractError, ContractResult};

#[cw_serde]
pub enum GuardState {
    Inactive,
    Active,
}

/// Contracts we call from Credit Manager should not be attempting to execute actions.
/// This prevents reentrancy attacks where a contract we call (that turned evil) deposits
/// into their own credit account and trick our state updates like update_coin_balances.rs which
/// rely on pre-post querying of bank balances of Rover.
/// NOTE: https://twitter.com/larry0x/status/1595919149381079041
pub struct ReentrancyGuard<'a>(Item<'a, GuardState>);

impl<'a> ReentrancyGuard<'a> {
    pub const fn new(namespace: &'a str) -> Self {
        Self(Item::new(namespace))
    }

    /// Ensures the guard has not already been set and sets to active
    pub fn check_and_set(&self, storage: &mut dyn Storage) -> ContractResult<()> {
        self.check_is_inactive(storage)?;
        self.update(storage, GuardState::Active)?;
        Ok(())
    }

    /// Sets guard to inactive and returns response to be used for callback
    pub fn remove<C>(&self, storage: &mut dyn Storage) -> ContractResult<Response<C>>
    where
        C: Clone + Debug + PartialEq + JsonSchema,
    {
        self.update(storage, GuardState::Inactive)?;
        Ok(Response::new().add_attribute("action", "remove_reentrancy_guard"))
    }

    fn check_is_inactive(&self, storage: &mut dyn Storage) -> ContractResult<()> {
        match self.state(storage)? {
            GuardState::Active => {
                Err(ContractError::ReentrancyGuard("Reentrancy guard is active".to_string()))
            }
            GuardState::Inactive => Ok(()),
        }
    }

    fn state(&self, storage: &dyn Storage) -> StdResult<GuardState> {
        Ok(self.0.may_load(storage)?.unwrap_or(GuardState::Inactive))
    }

    fn update(&self, storage: &mut dyn Storage, new_state: GuardState) -> ContractResult<()> {
        let new_state = self.transition_state(storage, new_state)?;
        Ok(self.0.save(storage, &new_state)?)
    }

    fn transition_state(
        &self,
        storage: &mut dyn Storage,
        new_state: GuardState,
    ) -> ContractResult<GuardState> {
        let current_state = self.state(storage)?;

        match (current_state, new_state) {
            (GuardState::Active, GuardState::Inactive) => Ok(GuardState::Inactive),
            (GuardState::Inactive, GuardState::Active) => Ok(GuardState::Active),
            _ => Err(ContractError::ReentrancyGuard(
                "Invalid reentrancy guard state transition".to_string(),
            )),
        }
    }
}
