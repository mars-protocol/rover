use cosmwasm_std::Addr;
use mars_rover::adapters::vault::VaultPositionType;

use mars_rover::error::ContractError::ExceedsMaxLiquidationLimit;
use mars_rover::msg::execute::Action::{LiquidateCoin, LiquidateVault};

use crate::helpers::{assert_err, uatom_info, unlocked_vault_info, uosmo_info, MockEnv};

pub mod helpers;

#[test]
fn test_only_one_liquidation_at_a_time() {
    let uosmo_info = uosmo_info();
    let uatom_info = uatom_info();
    let leverage_vault = unlocked_vault_info();

    let mut mock = MockEnv::new()
        .allowed_vaults(&[leverage_vault.clone()])
        .build()
        .unwrap();
    let liquidatee = Addr::unchecked("liquidatee");
    let liquidatee_account_id = mock.create_credit_account(&liquidatee).unwrap();

    let liquidator = Addr::unchecked("liquidator");
    let liquidator_account_id = mock.create_credit_account(&liquidator).unwrap();

    let vault = mock.get_vault(&leverage_vault);

    // two coin liquidations
    let res = mock.update_credit_account(
        &liquidator_account_id,
        &liquidator,
        vec![
            LiquidateCoin {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uatom_info.to_coin(10),
                request_coin_denom: uosmo_info.denom.clone(),
            },
            LiquidateCoin {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uatom_info.to_coin(10),
                request_coin_denom: uosmo_info.denom.clone(),
            },
        ],
        &[],
    );

    assert_err(res, ExceedsMaxLiquidationLimit);

    // two vault liquidations
    let res = mock.update_credit_account(
        &liquidator_account_id,
        &liquidator,
        vec![
            LiquidateVault {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uatom_info.to_coin(10),
                request_vault: vault.clone(),
                position_type: VaultPositionType::UNLOCKED,
            },
            LiquidateVault {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uatom_info.to_coin(10),
                request_vault: vault.clone(),
                position_type: VaultPositionType::UNLOCKED,
            },
        ],
        &[],
    );

    assert_err(res, ExceedsMaxLiquidationLimit);

    // one of each
    let res = mock.update_credit_account(
        &liquidator_account_id,
        &liquidator,
        vec![
            LiquidateVault {
                liquidatee_account_id: liquidatee_account_id.clone(),
                debt_coin: uatom_info.to_coin(10),
                request_vault: vault,
                position_type: VaultPositionType::UNLOCKED,
            },
            LiquidateCoin {
                liquidatee_account_id,
                debt_coin: uatom_info.to_coin(10),
                request_coin_denom: uosmo_info.denom,
            },
        ],
        &[],
    );

    assert_err(res, ExceedsMaxLiquidationLimit)
}
