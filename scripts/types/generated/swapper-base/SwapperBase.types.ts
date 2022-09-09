// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.16.3.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export interface InstantiateMsg {
  owner: string
  [k: string]: unknown
}
export type ExecuteMsg =
  | {
      update_config: {
        owner?: string | null
        [k: string]: unknown
      }
    }
  | {
      set_route: {
        denom_in: string
        denom_out: string
        route: Empty
        [k: string]: unknown
      }
    }
  | {
      swap_exact_in: {
        coin_in: Coin
        denom_out: string
        slippage: Decimal
        [k: string]: unknown
      }
    }
  | {
      transfer_result: {
        denom_in: string
        denom_out: string
        recipient: Addr
        [k: string]: unknown
      }
    }
export type Uint128 = string
export type Decimal = string
export type Addr = string
export interface Empty {
  [k: string]: unknown
}
export interface Coin {
  amount: Uint128
  denom: string
  [k: string]: unknown
}
export type QueryMsg =
  | {
      config: {}
    }
  | {
      route: {
        denom_in: string
        denom_out: string
      }
    }
  | {
      routes: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      estimate_exact_in_swap: {
        coin_in: Coin
        denom_out: string
      }
    }
export interface ConfigForString {
  owner: string
  [k: string]: unknown
}
export interface EstimateExactInSwapResponse {
  amount: Uint128
  [k: string]: unknown
}
export interface RouteResponseForEmpty {
  denom_in: string
  denom_out: string
  route: Empty
  [k: string]: unknown
}
export type ArrayOfRouteResponseForEmpty = RouteResponseForEmpty[]
