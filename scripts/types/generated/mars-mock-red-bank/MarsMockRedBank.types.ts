// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export interface InstantiateMsg {
  [k: string]: unknown
}
export type ExecuteMsg =
  | {
      update_owner: OwnerUpdate
    }
  | {
      update_emergency_owner: OwnerUpdate
    }
  | {
      update_config: {
        config: CreateOrUpdateConfig
      }
    }
  | {
      init_asset: {
        denom: string
        params: InitOrUpdateAssetParams
      }
    }
  | {
      update_asset: {
        denom: string
        params: InitOrUpdateAssetParams
      }
    }
  | {
      update_uncollateralized_loan_limit: {
        denom: string
        new_limit: Uint128
        user: string
      }
    }
  | {
      deposit: {
        on_behalf_of?: string | null
      }
    }
  | {
      withdraw: {
        amount?: Uint128 | null
        denom: string
        recipient?: string | null
      }
    }
  | {
      borrow: {
        amount: Uint128
        denom: string
        recipient?: string | null
      }
    }
  | {
      repay: {
        on_behalf_of?: string | null
      }
    }
  | {
      liquidate: {
        collateral_denom: string
        recipient?: string | null
        user: string
      }
    }
  | {
      update_asset_collateral_status: {
        denom: string
        enable: boolean
      }
    }
export type OwnerUpdate =
  | {
      propose_new_owner: {
        proposed: string
      }
    }
  | 'clear_proposed'
  | 'accept_proposed'
  | 'abolish_owner_role'
export type Decimal = string
export type Uint128 = string
export interface CreateOrUpdateConfig {
  address_provider?: string | null
  close_factor?: Decimal | null
}
export interface InitOrUpdateAssetParams {
  borrow_enabled?: boolean | null
  deposit_cap?: Uint128 | null
  deposit_enabled?: boolean | null
  interest_rate_model?: InterestRateModel | null
  liquidation_bonus?: Decimal | null
  liquidation_threshold?: Decimal | null
  max_loan_to_value?: Decimal | null
  reserve_factor?: Decimal | null
}
export interface InterestRateModel {
  base: Decimal
  optimal_utilization_rate: Decimal
  slope_1: Decimal
  slope_2: Decimal
}
export type QueryMsg =
  | {
      config: {}
    }
  | {
      market: {
        denom: string
      }
    }
  | {
      markets: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      uncollateralized_loan_limit: {
        denom: string
        user: string
      }
    }
  | {
      uncollateralized_loan_limits: {
        limit?: number | null
        start_after?: string | null
        user: string
      }
    }
  | {
      user_debt: {
        denom: string
        user: string
      }
    }
  | {
      user_debts: {
        limit?: number | null
        start_after?: string | null
        user: string
      }
    }
  | {
      user_collateral: {
        denom: string
        user: string
      }
    }
  | {
      user_collaterals: {
        limit?: number | null
        start_after?: string | null
        user: string
      }
    }
  | {
      user_position: {
        user: string
      }
    }
  | {
      scaled_liquidity_amount: {
        amount: Uint128
        denom: string
      }
    }
  | {
      scaled_debt_amount: {
        amount: Uint128
        denom: string
      }
    }
  | {
      underlying_liquidity_amount: {
        amount_scaled: Uint128
        denom: string
      }
    }
  | {
      underlying_debt_amount: {
        amount_scaled: Uint128
        denom: string
      }
    }
export interface ConfigResponse {
  address_provider: string
  close_factor: Decimal
  emergency_owner?: string | null
  owner?: string | null
  proposed_new_emergency_owner?: string | null
  proposed_new_owner?: string | null
}
export interface Market {
  borrow_enabled: boolean
  borrow_index: Decimal
  borrow_rate: Decimal
  collateral_total_scaled: Uint128
  debt_total_scaled: Uint128
  denom: string
  deposit_cap: Uint128
  deposit_enabled: boolean
  indexes_last_updated: number
  interest_rate_model: InterestRateModel
  liquidation_bonus: Decimal
  liquidation_threshold: Decimal
  liquidity_index: Decimal
  liquidity_rate: Decimal
  max_loan_to_value: Decimal
  reserve_factor: Decimal
}
export type ArrayOfMarket = Market[]
export interface UncollateralizedLoanLimitResponse {
  denom: string
  limit: Uint128
}
export type ArrayOfUncollateralizedLoanLimitResponse = UncollateralizedLoanLimitResponse[]
export interface UserCollateralResponse {
  amount: Uint128
  amount_scaled: Uint128
  denom: string
  enabled: boolean
}
export type ArrayOfUserCollateralResponse = UserCollateralResponse[]
export interface UserDebtResponse {
  amount: Uint128
  amount_scaled: Uint128
  denom: string
  uncollateralized: boolean
}
export type ArrayOfUserDebtResponse = UserDebtResponse[]
export type UserHealthStatus =
  | 'not_borrowing'
  | {
      borrowing: {
        liq_threshold_hf: Decimal
        max_ltv_hf: Decimal
      }
    }
export interface UserPositionResponse {
  health_status: UserHealthStatus
  total_collateralized_debt: Uint128
  total_enabled_collateral: Uint128
  weighted_liquidation_threshold_collateral: Uint128
  weighted_max_ltv_collateral: Uint128
}
