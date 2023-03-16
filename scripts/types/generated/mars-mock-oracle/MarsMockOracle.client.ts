// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.25.2.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { Coin, StdFee } from '@cosmjs/amino'
import {
  Decimal,
  InstantiateMsg,
  CoinPrice,
  ExecuteMsg,
  QueryMsg,
  PriceResponse,
} from './MarsMockOracle.types'
export interface MarsMockOracleReadOnlyInterface {
  contractAddress: string
  price: ({ denom }: { denom: string }) => Promise<PriceResponse>
}
export class MarsMockOracleQueryClient implements MarsMockOracleReadOnlyInterface {
  client: CosmWasmClient
  contractAddress: string

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client
    this.contractAddress = contractAddress
    this.price = this.price.bind(this)
  }

  price = async ({ denom }: { denom: string }): Promise<PriceResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      price: {
        denom,
      },
    })
  }
}
export interface MarsMockOracleInterface extends MarsMockOracleReadOnlyInterface {
  contractAddress: string
  sender: string
  changePrice: (
    {
      denom,
      price,
    }: {
      denom: string
      price: Decimal
    },
    fee?: number | StdFee | 'auto',
    memo?: string,
    funds?: Coin[],
  ) => Promise<ExecuteResult>
}
export class MarsMockOracleClient
  extends MarsMockOracleQueryClient
  implements MarsMockOracleInterface
{
  client: SigningCosmWasmClient
  sender: string
  contractAddress: string

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress)
    this.client = client
    this.sender = sender
    this.contractAddress = contractAddress
    this.changePrice = this.changePrice.bind(this)
  }

  changePrice = async (
    {
      denom,
      price,
    }: {
      denom: string
      price: Decimal
    },
    fee: number | StdFee | 'auto' = 'auto',
    memo?: string,
    funds?: Coin[],
  ): Promise<ExecuteResult> => {
    return await this.client.execute(
      this.sender,
      this.contractAddress,
      {
        change_price: {
          denom,
          price,
        },
      },
      fee,
      memo,
      funds,
    )
  }
}
