// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.23.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  OracleBaseForString,
  InstantiateMsg,
  LpConfig,
  ExecuteMsg,
  Uint128,
  QueryMsg,
  Coin,
  ArrayOfCoin,
} from './MarsMockZapper.types'
export interface MarsMockZapperReadOnlyInterface {
  contractAddress: string
  estimateProvideLiquidity: ({
    coinsIn,
    lpTokenOut,
  }: {
    coinsIn: Coin[]
    lpTokenOut: string
  }) => Promise<Uint128>
  estimateWithdrawLiquidity: ({ coinIn }: { coinIn: Coin }) => Promise<ArrayOfCoin>
}
export class MarsMockZapperQueryClient implements MarsMockZapperReadOnlyInterface {
  client: CosmWasmClient
  contractAddress: string

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client
    this.contractAddress = contractAddress
    this.estimateProvideLiquidity = this.estimateProvideLiquidity.bind(this)
    this.estimateWithdrawLiquidity = this.estimateWithdrawLiquidity.bind(this)
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
  estimateWithdrawLiquidity = async ({ coinIn }: { coinIn: Coin }): Promise<ArrayOfCoin> => {
    return this.client.queryContractSmart(this.contractAddress, {
      estimate_withdraw_liquidity: {
        coin_in: coinIn,
      },
    })
  }
}
export interface MarsMockZapperInterface extends MarsMockZapperReadOnlyInterface {
  contractAddress: string
  sender: string
  provideLiquidity: (
    {
      lpTokenOut,
      minimumReceive,
      recipient,
    }: {
      lpTokenOut: string
      minimumReceive: Uint128
      recipient?: string
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
  withdrawLiquidity: (
    {
      recipient,
    }: {
      recipient?: string
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
}
export class MarsMockZapperClient
  extends MarsMockZapperQueryClient
  implements MarsMockZapperInterface
{
  client: SigningCosmWasmClient
  sender: string
  contractAddress: string

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress)
    this.client = client
    this.sender = sender
    this.contractAddress = contractAddress
    this.provideLiquidity = this.provideLiquidity.bind(this)
    this.withdrawLiquidity = this.withdrawLiquidity.bind(this)
  }

  provideLiquidity = async (
    {
      lpTokenOut,
      minimumReceive,
      recipient,
    }: {
      lpTokenOut: string
      minimumReceive: Uint128
      recipient?: string
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        provide_liquidity: {
          lp_token_out: lpTokenOut,
          minimum_receive: minimumReceive,
          recipient,
        },
      },
      fee,
      memo,
      funds,
    )
  }
  withdrawLiquidity = async (
    {
      recipient,
    }: {
      recipient?: string
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        withdraw_liquidity: {
          recipient,
        },
      },
      fee,
      memo,
      funds,
    )
  }
}
