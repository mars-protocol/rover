use cosmwasm_std::{Decimal, DepsMut, MessageInfo, Response, Uint128};

use mars_rover::{
    adapters::vault::VaultUnchecked,
    error::ContractResult,
    msg::execute::EmergencyUpdate,
};

use crate::state::{OWNER, VAULT_CONFIGS};

pub fn emergency_config_update(
    deps: DepsMut,
    info: MessageInfo,
    update: EmergencyUpdate,
) -> ContractResult<Response> {
    OWNER.assert_emergency_owner(deps.storage, &info.sender)?;

    match update {
        EmergencyUpdate::SetZeroMaxLtv(v) => set_zero_max_ltv(deps, v),
        EmergencyUpdate::SetZeroDepositCap(v) => set_zero_deposit_cap(deps, v),
    }
}

pub fn set_zero_max_ltv(deps: DepsMut, v: VaultUnchecked) -> ContractResult<Response> {
    let vault = deps.api.addr_validate(&v.address)?;
    let mut config = VAULT_CONFIGS.load(deps.storage, &vault)?;
    config.max_ltv = Decimal::zero();
    VAULT_CONFIGS.save(deps.storage, &vault, &config)?;

    Ok(Response::new()
        .add_attribute("action", "set_zero_max_ltv")
        .add_attribute("vault", v.address))
}

pub fn set_zero_deposit_cap(deps: DepsMut, v: VaultUnchecked) -> ContractResult<Response> {
    let vault = deps.api.addr_validate(&v.address)?;
    let mut config = VAULT_CONFIGS.load(deps.storage, &vault)?;
    config.deposit_cap.amount = Uint128::zero();
    VAULT_CONFIGS.save(deps.storage, &vault, &config)?;

    Ok(Response::new()
        .add_attribute("action", "set_zero_deposit_cap")
        .add_attribute("vault", v.address))
}
