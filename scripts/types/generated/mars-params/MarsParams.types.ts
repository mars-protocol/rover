// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.30.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

export type Decimal = string
export interface InstantiateMsg {
  max_close_factor: Decimal
  owner: string
}
export type ExecuteMsg =
  | {
      update_owner: OwnerUpdate
    }
  | {
      update_max_close_factor: Decimal
    }
  | {
      update_asset_params: AssetParamsUpdate
    }
  | {
      update_vault_config: VaultConfigUpdate
    }
  | {
      emergency_update: EmergencyUpdate
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
  | {
      set_emergency_owner: {
        emergency_owner: string
      }
    }
  | 'clear_emergency_owner'
export type AssetParamsUpdate = {
  add_or_update: {
    denom: string
    params: AssetParams
  }
}
export type Uint128 = string
export type VaultConfigUpdate =
  | {
      add_or_update: {
        addr: string
        config: VaultConfig
      }
    }
  | {
      remove: {
        addr: string
      }
    }
export type EmergencyUpdate =
  | {
      rover: RoverEmergencyUpdate
    }
  | {
      red_bank: RedBankEmergencyUpdate
    }
export type RoverEmergencyUpdate =
  | {
      set_zero_max_ltv_on_vault: string
    }
  | {
      set_zero_deposit_cap_on_vault: string
    }
  | {
      disallow_coin: string
    }
export type RedBankEmergencyUpdate = {
  disable_borrowing: string
}
export interface AssetParams {
  liquidation_bonus: Decimal
  liquidation_threshold: Decimal
  max_loan_to_value: Decimal
  red_bank: RedBankSettings
  rover: RoverSettings
}
export interface RedBankSettings {
  borrow_enabled: boolean
  deposit_cap: Uint128
  deposit_enabled: boolean
}
export interface RoverSettings {
  hls: HighLeverageStrategyParams
  whitelisted: boolean
}
export interface HighLeverageStrategyParams {
  liquidation_threshold: Decimal
  max_loan_to_value: Decimal
}
export interface VaultConfig {
  deposit_cap: Coin
  liquidation_threshold: Decimal
  max_loan_to_value: Decimal
  whitelisted: boolean
}
export interface Coin {
  amount: Uint128
  denom: string
  [k: string]: unknown
}
export type QueryMsg =
  | {
      owner: {}
    }
  | {
      asset_params: {
        denom: string
      }
    }
  | {
      all_asset_params: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      vault_config: {
        address: string
      }
    }
  | {
      all_vault_configs: {
        limit?: number | null
        start_after?: string | null
      }
    }
  | {
      max_close_factor: {}
    }
export type ArrayOfAssetParamsResponse = AssetParamsResponse[]
export interface AssetParamsResponse {
  denom: string
  params: AssetParams
}
export type ArrayOfVaultConfig = VaultConfig[]
export interface OwnerResponse {
  abolished: boolean
  emergency_owner?: string | null
  initialized: boolean
  owner?: string | null
  proposed?: string | null
}
