// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export type Decimal = string
export interface InstantiateMsg {
  coins: CoinMarketInfo[]
}
export interface CoinMarketInfo {
  denom: string
  liquidation_threshold: Decimal
  max_ltv: Decimal
}
export type ExecuteMsg =
  | {
      borrow: {
        coin: Coin
        recipient?: string | null
      }
    }
  | {
      repay: {
        denom: string
        on_behalf_of?: string | null
      }
    }
export type Uint128 = string
export interface Coin {
  amount: Uint128
  denom: string
  [k: string]: unknown
}
export type QueryMsg =
  | {
      user_asset_debt: {
        denom: string
        user_address: string
      }
    }
  | {
      market: {
        denom: string
      }
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
  [k: string]: unknown
}
export interface InterestRateModel {
  base: Decimal
  optimal_utilization_rate: Decimal
  slope_1: Decimal
  slope_2: Decimal
  [k: string]: unknown
}
export interface UserAssetDebtResponse {
  amount: Uint128
  denom: string
}