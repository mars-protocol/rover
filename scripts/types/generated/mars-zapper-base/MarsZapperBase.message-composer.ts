// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.33.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { MsgExecuteContractEncodeObject } from '@cosmjs/cosmwasm-stargate'
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx'
import { toUtf8 } from '@cosmjs/encoding'
import {
  InstantiateMsg,
  ExecuteMsg,
  Uint128,
  CallbackMsg,
  Addr,
  Coin,
  QueryMsg,
  ArrayOfCoin,
} from './MarsZapperBase.types'
export interface MarsZapperBaseMessage {
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
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  withdrawLiquidity: (
    {
      minimumReceive,
      recipient,
    }: {
      minimumReceive: Coin[]
      recipient?: string
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  callback: (callbackMsg: CallbackMsg, _funds?: Coin[]) => MsgExecuteContractEncodeObject
}
export class MarsZapperBaseMessageComposer implements MarsZapperBaseMessage {
  sender: string
  contractAddress: string

  constructor(sender: string, contractAddress: string) {
    this.sender = sender
    this.contractAddress = contractAddress
    this.provideLiquidity = this.provideLiquidity.bind(this)
    this.withdrawLiquidity = this.withdrawLiquidity.bind(this)
    this.callback = this.callback.bind(this)
  }

  provideLiquidity = (
    {
      lpTokenOut,
      minimumReceive,
      recipient,
    }: {
      lpTokenOut: string
      minimumReceive: Uint128
      recipient?: string
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
            provide_liquidity: {
              lp_token_out: lpTokenOut,
              minimum_receive: minimumReceive,
              recipient,
            },
          }),
        ),
        funds: _funds,
      }),
    }
  }
  withdrawLiquidity = (
    {
      minimumReceive,
      recipient,
    }: {
      minimumReceive: Coin[]
      recipient?: string
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
            withdraw_liquidity: {
              minimum_receive: minimumReceive,
              recipient,
            },
          }),
        ),
        funds: _funds,
      }),
    }
  }
  callback = (callbackMsg: CallbackMsg, _funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            callback: callbackMsg,
          }),
        ),
        funds: _funds,
      }),
    }
  }
}
