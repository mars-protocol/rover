use cosmwasm_std::{
    testing::{mock_dependencies, mock_env},
    Addr,
};
use cw2::VersionError;
use mars_credit_manager::{
    contract::{migrate, CONTRACT_NAME},
    migrations::v2_0_0::{v1_owner, v1_owner::OwnerSetNoneProposed},
    state::{HEALTH_CONTRACT, INCENTIVES, OWNER, PARAMS, REWARDS_COLLECTOR, SWAPPER},
};
use mars_rover::{
    adapters::{
        health::HealthContractUnchecked, incentives::IncentivesUnchecked, params::ParamsUnchecked,
        rewards_collector::RewardsCollector, swap::SwapperUnchecked,
    },
    error::ContractError,
    msg::{migrate::V2Updates, MigrateMsg},
};

pub mod helpers;

#[test]
fn wrong_contract_name() {
    let mut deps = mock_dependencies();
    cw2::set_contract_version(deps.as_mut().storage, "contract_xyz", "1.0.0").unwrap();

    let err = migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV2_0_0(V2Updates {
            health_contract: HealthContractUnchecked::new("health".to_string()),
            params: ParamsUnchecked::new("params".to_string()),
            incentives: IncentivesUnchecked::new("incentives".to_string()),
            swapper: SwapperUnchecked::new("swapper".to_string()),
            rewards_collector: RewardsCollector {
                address: "xyz".to_string(),
                account_id: "123".to_string(),
            },
        }),
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::Version(VersionError::WrongContract {
            expected: "mars-credit-manager".to_string(),
            found: "contract_xyz".to_string()
        })
    );
}

#[test]
fn wrong_contract_version() {
    let mut deps = mock_dependencies();
    cw2::set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "4.1.0").unwrap();

    let err = migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV2_0_0(V2Updates {
            health_contract: HealthContractUnchecked::new("health".to_string()),
            params: ParamsUnchecked::new("params".to_string()),
            incentives: IncentivesUnchecked::new("incentives".to_string()),
            swapper: SwapperUnchecked::new("swapper".to_string()),
            rewards_collector: RewardsCollector {
                address: "xyz".to_string(),
                account_id: "123".to_string(),
            },
        }),
    )
    .unwrap_err();

    assert_eq!(
        err,
        ContractError::Version(VersionError::WrongVersion {
            expected: "1.0.0".to_string(),
            found: "4.1.0".to_string()
        })
    );
}

#[test]
fn successful_migration() {
    let mut deps = mock_dependencies();
    cw2::set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "1.0.0").unwrap();

    let old_owner = "spiderman_246";
    v1_owner::OWNER
        .save(
            deps.as_mut().storage,
            &v1_owner::OwnerState::B(OwnerSetNoneProposed {
                owner: Addr::unchecked(old_owner),
            }),
        )
        .unwrap();

    let health_contract = "health_addr_123".to_string();
    let params = "params_addr_456".to_string();
    let incentives = "incentives_addr_789".to_string();
    let swapper = "swapper_addr_012".to_string();
    let rewards_collector = RewardsCollector {
        address: "rewards_collector_addr".to_string(),
        account_id: "4117".to_string(),
    };

    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV2_0_0(V2Updates {
            health_contract: HealthContractUnchecked::new(health_contract.clone()),
            params: ParamsUnchecked::new(params.clone()),
            incentives: IncentivesUnchecked::new(incentives.clone()),
            swapper: SwapperUnchecked::new(swapper.clone()),
            rewards_collector: rewards_collector.clone(),
        }),
    )
    .unwrap();

    let set_health_contract =
        HEALTH_CONTRACT.load(deps.as_ref().storage).unwrap().address().to_string();
    assert_eq!(health_contract, set_health_contract);

    let set_params = PARAMS.load(deps.as_ref().storage).unwrap().address().to_string();
    assert_eq!(params, set_params);

    let set_incentives = INCENTIVES.load(deps.as_ref().storage).unwrap().addr.to_string();
    assert_eq!(incentives, set_incentives);

    let set_swapper = SWAPPER.load(deps.as_ref().storage).unwrap().address().to_string();
    assert_eq!(swapper, set_swapper);

    let set_rewards = REWARDS_COLLECTOR.load(deps.as_ref().storage).unwrap();
    assert_eq!(rewards_collector, set_rewards);

    let o = OWNER.query(deps.as_ref().storage).unwrap();
    assert_eq!(old_owner.to_string(), o.owner.unwrap());
    assert!(o.proposed.is_none());
    assert!(o.initialized);
    assert!(!o.abolished);
    assert!(o.emergency_owner.is_none());
}