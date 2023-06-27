// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.30.1.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { MsgExecuteContractEncodeObject } from '@cosmjs/cosmwasm-stargate'
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx'
import { toUtf8 } from '@cosmjs/encoding'
import {
  InstantiateMsg,
  ExecuteMsg,
  OwnerUpdate,
  OsmosisRoute,
  Uint128,
  Decimal,
  Addr,
  SwapAmountInRoute,
  Coin,
  QueryMsg,
  EstimateExactInSwapResponse,
  OwnerResponse,
  RouteResponseForEmpty,
  Empty,
  ArrayOfRouteResponseForEmpty,
} from './MarsSwapperOsmosis.types'
export interface MarsSwapperOsmosisMessage {
  contractAddress: string
  sender: string
  updateOwner: (ownerUpdate: OwnerUpdate, _funds?: Coin[]) => MsgExecuteContractEncodeObject
  setRoute: (
    {
      denomIn,
      denomOut,
      route,
    }: {
      denomIn: string
      denomOut: string
      route: OsmosisRoute
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  swapExactIn: (
    {
      coinIn,
      denomOut,
      slippage,
    }: {
      coinIn: Coin
      denomOut: string
      slippage: Decimal
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  transferResult: (
    {
      denomIn,
      denomOut,
      recipient,
    }: {
      denomIn: string
      denomOut: string
      recipient: Addr
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
}
export class MarsSwapperOsmosisMessageComposer implements MarsSwapperOsmosisMessage {
  sender: string
  contractAddress: string

  constructor(sender: string, contractAddress: string) {
    this.sender = sender
    this.contractAddress = contractAddress
    this.updateOwner = this.updateOwner.bind(this)
    this.setRoute = this.setRoute.bind(this)
    this.swapExactIn = this.swapExactIn.bind(this)
    this.transferResult = this.transferResult.bind(this)
  }

  updateOwner = (ownerUpdate: OwnerUpdate, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            update_owner: ownerUpdate,
          }),
        ),
        funds: _funds,
      }),
    }
  }
  setRoute = (
    {
      denomIn,
      denomOut,
      route,
    }: {
      denomIn: string
      denomOut: string
      route: OsmosisRoute
    },
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            set_route: {
              denom_in: denomIn,
              denom_out: denomOut,
              route,
            },
          }),
        ),
        funds: _funds,
      }),
    }
  }
  swapExactIn = (
    {
      coinIn,
      denomOut,
      slippage,
    }: {
      coinIn: Coin
      denomOut: string
      slippage: Decimal
    },
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            swap_exact_in: {
              coin_in: coinIn,
              denom_out: denomOut,
              slippage,
            },
          }),
        ),
        funds: _funds,
      }),
    }
  }
  transferResult = (
    {
      denomIn,
      denomOut,
      recipient,
    }: {
      denomIn: string
      denomOut: string
      recipient: Addr
    },
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            transfer_result: {
              denom_in: denomIn,
              denom_out: denomOut,
              recipient,
            },
          }),
        ),
        funds: _funds,
      }),
    }
  }
}