use cosmwasm_std::DepsMut;

use mars_rover::error::ContractResult;
use mars_rover::msg::InstantiateMsg;

use crate::state::{
    ALLOWED_COINS, MAX_CLOSE_FACTOR, ORACLE, OWNER, RED_BANK, SWAPPER, VAULT_CONFIGS, ZAPPER,
};

pub fn store_config(deps: DepsMut, msg: &InstantiateMsg) -> ContractResult<()> {
    let owner = deps.api.addr_validate(&msg.owner)?;
    OWNER.save(deps.storage, &owner)?;
    RED_BANK.save(deps.storage, &msg.red_bank.check(deps.api)?)?;
    ORACLE.save(deps.storage, &msg.oracle.check(deps.api)?)?;
    MAX_CLOSE_FACTOR.save(deps.storage, &msg.max_close_factor)?;
    SWAPPER.save(deps.storage, &msg.swapper.check(deps.api)?)?;
    ZAPPER.save(deps.storage, &msg.zapper.check(deps.api)?)?;

    msg.allowed_vaults
        .iter()
        .try_for_each(|v| -> ContractResult<_> {
            v.config.check()?;
            let vault = v.vault.check(deps.api)?;
            Ok(VAULT_CONFIGS.save(deps.storage, &vault.address, &v.config)?)
        })?;

    msg.allowed_coins
        .iter()
        .try_for_each(|denom| ALLOWED_COINS.insert(deps.storage, denom).map(|_| ()))?;

    Ok(())
}
