// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  Decimal,
  Uint128,
  OracleBaseForString,
  RedBankBaseForString,
  SwapperBaseForString,
  ZapperBaseForString,
  InstantiateMsg,
  VaultInstantiateConfig,
  VaultConfig,
  Coin,
  VaultBaseForString,
  ExecuteMsg,
  Action,
  ActionAmount,
  VaultPositionType,
  AdminUpdate,
  CallbackMsg,
  Addr,
  ActionCoin,
  ConfigUpdates,
  VaultBaseForAddr,
  QueryMsg,
  ArrayOfCoinBalanceResponseItem,
  CoinBalanceResponseItem,
  ArrayOfSharesResponseItem,
  SharesResponseItem,
  ArrayOfDebtShares,
  DebtShares,
  ArrayOfVaultWithBalance,
  VaultWithBalance,
  VaultPositionAmount,
  VaultAmount,
  VaultAmount1,
  UnlockingPositions,
  ArrayOfVaultPositionResponseItem,
  VaultPositionResponseItem,
  VaultPosition,
  LockingVaultAmount,
  VaultUnlockingPosition,
  ArrayOfString,
  ConfigResponse,
  ArrayOfCoin,
  HealthResponse,
  Positions,
  DebtAmount,
  ArrayOfVaultInstantiateConfig,
} from './MarsCreditManager.types'
export interface MarsCreditManagerReadOnlyInterface {
  contractAddress: string
  config: () => Promise<ConfigResponse>
  vaultConfigs: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: VaultBaseForString
  }) => Promise<ArrayOfVaultInstantiateConfig>
  allowedCoins: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string
  }) => Promise<ArrayOfString>
  positions: ({ accountId }: { accountId: string }) => Promise<Positions>
  health: ({ accountId }: { accountId: string }) => Promise<HealthResponse>
  allCoinBalances: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }) => Promise<ArrayOfCoinBalanceResponseItem>
  allDebtShares: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }) => Promise<ArrayOfSharesResponseItem>
  totalDebtShares: () => Promise<DebtShares>
  allTotalDebtShares: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string
  }) => Promise<ArrayOfDebtShares>
  allVaultPositions: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }) => Promise<ArrayOfVaultPositionResponseItem>
  totalVaultCoinBalance: ({ vault }: { vault: VaultBaseForString }) => Promise<Uint128>
  allTotalVaultCoinBalances: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: VaultBaseForString
  }) => Promise<ArrayOfVaultWithBalance>
  estimateProvideLiquidity: ({
    coinsIn,
    lpTokenOut,
  }: {
    coinsIn: Coin[]
    lpTokenOut: string
  }) => Promise<Uint128>
  estimateWithdrawLiquidity: ({ lpToken }: { lpToken: Coin }) => Promise<ArrayOfCoin>
}
export class MarsCreditManagerQueryClient implements MarsCreditManagerReadOnlyInterface {
  client: CosmWasmClient
  contractAddress: string

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client
    this.contractAddress = contractAddress
    this.config = this.config.bind(this)
    this.vaultConfigs = this.vaultConfigs.bind(this)
    this.allowedCoins = this.allowedCoins.bind(this)
    this.positions = this.positions.bind(this)
    this.health = this.health.bind(this)
    this.allCoinBalances = this.allCoinBalances.bind(this)
    this.allDebtShares = this.allDebtShares.bind(this)
    this.totalDebtShares = this.totalDebtShares.bind(this)
    this.allTotalDebtShares = this.allTotalDebtShares.bind(this)
    this.allVaultPositions = this.allVaultPositions.bind(this)
    this.totalVaultCoinBalance = this.totalVaultCoinBalance.bind(this)
    this.allTotalVaultCoinBalances = this.allTotalVaultCoinBalances.bind(this)
    this.estimateProvideLiquidity = this.estimateProvideLiquidity.bind(this)
    this.estimateWithdrawLiquidity = this.estimateWithdrawLiquidity.bind(this)
  }

  config = async (): Promise<ConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {},
    })
  }
  vaultConfigs = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: VaultBaseForString
  }): Promise<ArrayOfVaultInstantiateConfig> => {
    return this.client.queryContractSmart(this.contractAddress, {
      vault_configs: {
        limit,
        start_after: startAfter,
      },
    })
  }
  allowedCoins = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string
  }): Promise<ArrayOfString> => {
    return this.client.queryContractSmart(this.contractAddress, {
      allowed_coins: {
        limit,
        start_after: startAfter,
      },
    })
  }
  positions = async ({ accountId }: { accountId: string }): Promise<Positions> => {
    return this.client.queryContractSmart(this.contractAddress, {
      positions: {
        account_id: accountId,
      },
    })
  }
  health = async ({ accountId }: { accountId: string }): Promise<HealthResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      health: {
        account_id: accountId,
      },
    })
  }
  allCoinBalances = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }): Promise<ArrayOfCoinBalanceResponseItem> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_coin_balances: {
        limit,
        start_after: startAfter,
      },
    })
  }
  allDebtShares = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }): Promise<ArrayOfSharesResponseItem> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_debt_shares: {
        limit,
        start_after: startAfter,
      },
    })
  }
  totalDebtShares = async (): Promise<DebtShares> => {
    return this.client.queryContractSmart(this.contractAddress, {
      total_debt_shares: {},
    })
  }
  allTotalDebtShares = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string
  }): Promise<ArrayOfDebtShares> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_total_debt_shares: {
        limit,
        start_after: startAfter,
      },
    })
  }
  allVaultPositions = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }): Promise<ArrayOfVaultPositionResponseItem> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_vault_positions: {
        limit,
        start_after: startAfter,
      },
    })
  }
  totalVaultCoinBalance = async ({ vault }: { vault: VaultBaseForString }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      total_vault_coin_balance: {
        vault,
      },
    })
  }
  allTotalVaultCoinBalances = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: VaultBaseForString
  }): Promise<ArrayOfVaultWithBalance> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_total_vault_coin_balances: {
        limit,
        start_after: startAfter,
      },
    })
  }
  estimateProvideLiquidity = async ({
    coinsIn,
    lpTokenOut,
  }: {
    coinsIn: Coin[]
    lpTokenOut: string
  }): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      estimate_provide_liquidity: {
        coins_in: coinsIn,
        lp_token_out: lpTokenOut,
      },
    })
  }
  estimateWithdrawLiquidity = async ({ lpToken }: { lpToken: Coin }): Promise<ArrayOfCoin> => {
    return this.client.queryContractSmart(this.contractAddress, {
      estimate_withdraw_liquidity: {
        lp_token: lpToken,
      },
    })
  }
}
export interface MarsCreditManagerInterface extends MarsCreditManagerReadOnlyInterface {
  contractAddress: string
  sender: string
  createCreditAccount: (
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  updateCreditAccount: (
    {
      accountId,
      actions,
    }: {
      accountId: string
      actions: Action[]
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  updateConfig: (
    {
      newConfig,
    }: {
      newConfig: ConfigUpdates
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  updateAdmin: (
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  callback: (
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
}
export class MarsCreditManagerClient
  extends MarsCreditManagerQueryClient
  implements MarsCreditManagerInterface
{
  client: SigningCosmWasmClient
  sender: string
  contractAddress: string

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress)
    this.client = client
    this.sender = sender
    this.contractAddress = contractAddress
    this.createCreditAccount = this.createCreditAccount.bind(this)
    this.updateCreditAccount = this.updateCreditAccount.bind(this)
    this.updateConfig = this.updateConfig.bind(this)
    this.updateAdmin = this.updateAdmin.bind(this)
    this.callback = this.callback.bind(this)
  }

  createCreditAccount = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        create_credit_account: {},
      },
      fee,
      memo,
      funds,
    )
  }
  updateCreditAccount = async (
    {
      accountId,
      actions,
    }: {
      accountId: string
      actions: Action[]
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_credit_account: {
          account_id: accountId,
          actions,
        },
      },
      fee,
      memo,
      funds,
    )
  }
  updateConfig = async (
    {
      newConfig,
    }: {
      newConfig: ConfigUpdates
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_config: {
          new_config: newConfig,
        },
      },
      fee,
      memo,
      funds,
    )
  }
  updateAdmin = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        update_admin: {},
      },
      fee,
      memo,
      funds,
    )
  }
  callback = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        callback: {},
      },
      fee,
      memo,
      funds,
    )
  }
}
