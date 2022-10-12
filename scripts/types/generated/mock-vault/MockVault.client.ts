// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  OracleBaseForString,
  InstantiateMsg,
  ExecuteMsg,
  Uint128,
  QueryMsg,
  VaultInfo,
  ArrayOfCoin,
  Coin,
  Timestamp,
  Uint64,
  UnlockingPosition,
  ArrayOfUnlockingPosition,
} from './MockVault.types'
export interface MockVaultReadOnlyInterface {
  contractAddress: string
  info: () => Promise<VaultInfo>
  previewRedeem: ({ amount }: { amount: Uint128 }) => Promise<ArrayOfCoin>
  totalVaultCoinsIssued: () => Promise<Uint128>
  unlockingPositionsForAddr: ({ addr }: { addr: string }) => Promise<ArrayOfUnlockingPosition>
  unlockingPosition: ({ id }: { id: Uint128 }) => Promise<UnlockingPosition>
}
export class MockVaultQueryClient implements MockVaultReadOnlyInterface {
  client: CosmWasmClient
  contractAddress: string

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client
    this.contractAddress = contractAddress
    this.info = this.info.bind(this)
    this.previewRedeem = this.previewRedeem.bind(this)
    this.totalVaultCoinsIssued = this.totalVaultCoinsIssued.bind(this)
    this.unlockingPositionsForAddr = this.unlockingPositionsForAddr.bind(this)
    this.unlockingPosition = this.unlockingPosition.bind(this)
  }

  info = async (): Promise<VaultInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      info: {},
    })
  }
  previewRedeem = async ({ amount }: { amount: Uint128 }): Promise<ArrayOfCoin> => {
    return this.client.queryContractSmart(this.contractAddress, {
      preview_redeem: {
        amount,
      },
    })
  }
  totalVaultCoinsIssued = async (): Promise<Uint128> => {
    return this.client.queryContractSmart(this.contractAddress, {
      total_vault_coins_issued: {},
    })
  }
  unlockingPositionsForAddr = async ({
    addr,
  }: {
    addr: string
  }): Promise<ArrayOfUnlockingPosition> => {
    return this.client.queryContractSmart(this.contractAddress, {
      unlocking_positions_for_addr: {
        addr,
      },
    })
  }
  unlockingPosition = async ({ id }: { id: Uint128 }): Promise<UnlockingPosition> => {
    return this.client.queryContractSmart(this.contractAddress, {
      unlocking_position: {
        id,
      },
    })
  }
}
export interface MockVaultInterface extends MockVaultReadOnlyInterface {
  contractAddress: string
  sender: string
  deposit: (fee?: number | StdFee | 'auto', memo?: string, funds?: Coin[]) => Promise<ExecuteResult>
  withdraw: (
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  forceWithdraw: (
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  requestUnlock: (
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  withdrawUnlocked: (
    {
      id,
    }: {
      id: Uint128
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
}
export class MockVaultClient extends MockVaultQueryClient implements MockVaultInterface {
  client: SigningCosmWasmClient
  sender: string
  contractAddress: string

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress)
    this.client = client
    this.sender = sender
    this.contractAddress = contractAddress
    this.deposit = this.deposit.bind(this)
    this.withdraw = this.withdraw.bind(this)
    this.forceWithdraw = this.forceWithdraw.bind(this)
    this.requestUnlock = this.requestUnlock.bind(this)
    this.withdrawUnlocked = this.withdrawUnlocked.bind(this)
  }

  deposit = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        deposit: {},
      },
      fee,
      memo,
      funds,
    )
  }
  withdraw = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        withdraw: {},
      },
      fee,
      memo,
      funds,
    )
  }
  forceWithdraw = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        force_withdraw: {},
      },
      fee,
      memo,
      funds,
    )
  }
  requestUnlock = async (
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        request_unlock: {},
      },
      fee,
      memo,
      funds,
    )
  }
  withdrawUnlocked = async (
    {
      id,
    }: {
      id: Uint128
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        withdraw_unlocked: {
          id,
        },
      },
      fee,
      memo,
      funds,
    )
  }
}
