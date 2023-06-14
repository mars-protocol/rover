// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.30.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  InstantiateMsg,
  ExecuteMsg,
  Uint128,
  VaultPositionAmount,
  VaultAmount,
  VaultAmount1,
  UnlockingPositions,
  Addr,
  Positions,
  DebtAmount,
  Coin,
  LentAmount,
  VaultPosition,
  LockingVaultAmount,
  VaultUnlockingPosition,
  VaultBaseForAddr,
  QueryMsg,
  VaultBaseForString,
  AccountKind,
  ArrayOfCoinBalanceResponseItem,
  CoinBalanceResponseItem,
  ArrayOfSharesResponseItem,
  SharesResponseItem,
  ArrayOfDebtShares,
  DebtShares,
  ArrayOfLentShares,
  LentShares,
  ArrayOfVaultPositionResponseItem,
  VaultPositionResponseItem,
  ConfigResponse,
  OwnerResponse,
  ArrayOfCoin,
  VaultPositionValue,
  CoinValue,
  VaultUtilizationResponse,
} from './MarsMockCreditManager.types'
export interface MarsMockCreditManagerReadOnlyInterface {
  contractAddress: string
  accountKind: ({ accountId }: { accountId: string }) => Promise<AccountKind>
  config: () => Promise<ConfigResponse>
  vaultUtilization: ({ vault }: { vault: VaultBaseForString }) => Promise<VaultUtilizationResponse>
  positions: ({ accountId }: { accountId: string }) => Promise<Positions>
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
  allLentShares: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }) => Promise<ArrayOfSharesResponseItem>
  totalLentShares: () => Promise<LentShares>
  allTotalLentShares: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string
  }) => Promise<ArrayOfLentShares>
  allVaultPositions: ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }) => Promise<ArrayOfVaultPositionResponseItem>
  estimateProvideLiquidity: ({
    coinsIn,
    lpTokenOut,
  }: {
    coinsIn: Coin[]
    lpTokenOut: string
  }) => Promise<Uint128>
  estimateWithdrawLiquidity: ({ lpToken }: { lpToken: Coin }) => Promise<ArrayOfCoin>
  vaultPositionValue: ({
    vaultPosition,
  }: {
    vaultPosition: VaultPosition
  }) => Promise<VaultPositionValue>
}
export class MarsMockCreditManagerQueryClient implements MarsMockCreditManagerReadOnlyInterface {
  client: CosmWasmClient
  contractAddress: string

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client
    this.contractAddress = contractAddress
    this.accountKind = this.accountKind.bind(this)
    this.config = this.config.bind(this)
    this.vaultUtilization = this.vaultUtilization.bind(this)
    this.positions = this.positions.bind(this)
    this.allCoinBalances = this.allCoinBalances.bind(this)
    this.allDebtShares = this.allDebtShares.bind(this)
    this.totalDebtShares = this.totalDebtShares.bind(this)
    this.allTotalDebtShares = this.allTotalDebtShares.bind(this)
    this.allLentShares = this.allLentShares.bind(this)
    this.totalLentShares = this.totalLentShares.bind(this)
    this.allTotalLentShares = this.allTotalLentShares.bind(this)
    this.allVaultPositions = this.allVaultPositions.bind(this)
    this.estimateProvideLiquidity = this.estimateProvideLiquidity.bind(this)
    this.estimateWithdrawLiquidity = this.estimateWithdrawLiquidity.bind(this)
    this.vaultPositionValue = this.vaultPositionValue.bind(this)
  }

  accountKind = async ({ accountId }: { accountId: string }): Promise<AccountKind> => {
    return this.client.queryContractSmart(this.contractAddress, {
      account_kind: {
        account_id: accountId,
      },
    })
  }
  config = async (): Promise<ConfigResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {},
    })
  }
  vaultUtilization = async ({
    vault,
  }: {
    vault: VaultBaseForString
  }): Promise<VaultUtilizationResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      vault_utilization: {
        vault,
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
  allLentShares = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string[][]
  }): Promise<ArrayOfSharesResponseItem> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_lent_shares: {
        limit,
        start_after: startAfter,
      },
    })
  }
  totalLentShares = async (): Promise<LentShares> => {
    return this.client.queryContractSmart(this.contractAddress, {
      total_lent_shares: {},
    })
  }
  allTotalLentShares = async ({
    limit,
    startAfter,
  }: {
    limit?: number
    startAfter?: string
  }): Promise<ArrayOfLentShares> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_total_lent_shares: {
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
  vaultPositionValue = async ({
    vaultPosition,
  }: {
    vaultPosition: VaultPosition
  }): Promise<VaultPositionValue> => {
    return this.client.queryContractSmart(this.contractAddress, {
      vault_position_value: {
        vault_position: vaultPosition,
      },
    })
  }
}
export interface MarsMockCreditManagerInterface extends MarsMockCreditManagerReadOnlyInterface {
  contractAddress: string
  sender: string
  setPositionsResponse: (
    {
      accountId,
      positions,
    }: {
      accountId: string
      positions: Positions
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    _funds?: Coin[],
  ) => Promise<ExecuteResult>
}
export class MarsMockCreditManagerClient
  extends MarsMockCreditManagerQueryClient
  implements MarsMockCreditManagerInterface
{
  client: SigningCosmWasmClient
  sender: string
  contractAddress: string

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress)
    this.client = client
    this.sender = sender
    this.contractAddress = contractAddress
    this.setPositionsResponse = this.setPositionsResponse.bind(this)
  }

  setPositionsResponse = async (
    {
      accountId,
      positions,
    }: {
      accountId: string
      positions: Positions
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    _funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        set_positions_response: {
          account_id: accountId,
          positions,
        },
      },
      fee,
      memo,
      _funds,
    )
  }
}
