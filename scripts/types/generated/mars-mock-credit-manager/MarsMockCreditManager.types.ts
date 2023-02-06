// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export interface InstantiateMsg {
  [k: string]: unknown
}
export type ExecuteMsg =
  | {
      set_health_response: {
        account_id: string
        response: HealthResponse
      }
    }
  | {
      set_positions_response: {
        account_id: string
        positions: Positions
      }
    }
  | {
      set_allowed_coins: string[]
    }
  | {
      set_vault_config: {
        address: string
        config: VaultConfig
      }
    }
export type Decimal = string
export type Uint128 = string
export type VaultPositionAmount =
  | {
      unlocked: VaultAmount
    }
  | {
      locking: LockingVaultAmount
    }
export type VaultAmount = string
export type VaultAmount1 = string
export type UnlockingPositions = VaultUnlockingPosition[]
export type Addr = string
export interface HealthResponse {
  above_max_ltv: boolean
  liquidatable: boolean
  liquidation_health_factor?: Decimal | null
  liquidation_threshold_adjusted_collateral: Uint128
  max_ltv_adjusted_collateral: Uint128
  max_ltv_health_factor?: Decimal | null
  total_collateral_value: Uint128
  total_debt_value: Uint128
}
export interface Positions {
  account_id: string
  debts: DebtAmount[]
  deposits: Coin[]
  lends: LentAmount[]
  vaults: VaultPosition[]
}
export interface DebtAmount {
  amount: Uint128
  denom: string
  shares: Uint128
}
export interface Coin {
  amount: Uint128
  denom: string
  [k: string]: unknown
}
export interface LentAmount {
  amount: Uint128
  denom: string
  shares: Uint128
}
export interface VaultPosition {
  amount: VaultPositionAmount
  vault: VaultBaseForAddr
}
export interface LockingVaultAmount {
  locked: VaultAmount1
  unlocking: UnlockingPositions
}
export interface VaultUnlockingPosition {
  coin: Coin
  id: number
}
export interface VaultBaseForAddr {
  address: Addr
}
export interface VaultConfig {
  deposit_cap: Coin
  liquidation_threshold: Decimal
  max_ltv: Decimal
  whitelisted: boolean
}
export type QueryMsg =
  | {
      config: {}
    }
  | {
      vaults_info: {
        limit?: number | null
        start_after?: VaultBaseForString | null
      }
    }
  | {
      allowed_coins: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      positions: {
        account_id: string
      }
    }
  | {
      health: {
        account_id: string
      }
    }
  | {
      all_coin_balances: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      all_debt_shares: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      total_debt_shares: string
    }
  | {
      all_total_debt_shares: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      all_lent_shares: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      total_lent_shares: string
    }
  | {
      all_total_lent_shares: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      all_vault_positions: {
        limit?: number | null
        start_after?: [string, string] | null
      }
    }
  | {
      total_vault_coin_balance: {
        vault: VaultBaseForString
      }
    }
  | {
      all_total_vault_coin_balances: {
        limit?: number | null
        start_after?: VaultBaseForString | null
      }
    }
  | {
      estimate_provide_liquidity: {
        coins_in: Coin[]
        lp_token_out: string
      }
    }
  | {
      estimate_withdraw_liquidity: {
        lp_token: Coin
      }
    }
export interface VaultBaseForString {
  address: string
}
export type ArrayOfCoinBalanceResponseItem = CoinBalanceResponseItem[]
export interface CoinBalanceResponseItem {
  account_id: string
  amount: Uint128
  denom: string
}
export type ArrayOfSharesResponseItem = SharesResponseItem[]
export interface SharesResponseItem {
  account_id: string
  denom: string
  shares: Uint128
}
export type ArrayOfDebtShares = DebtShares[]
export interface DebtShares {
  denom: string
  shares: Uint128
}
export type ArrayOfLentShares = LentShares[]
export interface LentShares {
  denom: string
  shares: Uint128
}
export type ArrayOfVaultWithBalance = VaultWithBalance[]
export interface VaultWithBalance {
  balance: Uint128
  vault: VaultBaseForAddr
}
export type ArrayOfVaultPositionResponseItem = VaultPositionResponseItem[]
export interface VaultPositionResponseItem {
  account_id: string
  position: VaultPosition
}
export type ArrayOfString = string[]
export interface ConfigResponse {
  account_nft?: string | null
  max_close_factor: Decimal
  max_unlocking_positions: Uint128
  oracle: string
  owner?: string | null
  proposed_new_owner?: string | null
  red_bank: string
  swapper: string
  zapper: string
}
export type ArrayOfCoin = Coin[]
export type ArrayOfVaultInfoResponse = VaultInfoResponse[]
export interface VaultInfoResponse {
  config: VaultConfig
  utilization: Coin
  vault: VaultBaseForString
}
