// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.23.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { MsgExecuteContractEncodeObject } from 'cosmwasm'
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx'
import { toUtf8 } from '@cosmjs/encoding'
import {
  OracleBaseForString,
  Addr,
  PricingMethod,
  InstantiateMsg,
  VaultPricingInfo,
  ExecuteMsg,
  ConfigUpdates,
  QueryMsg,
  Uint128,
  Coin,
  ArrayOfVaultPricingInfo,
  OracleBaseForAddr,
  ConfigResponse,
  Decimal,
  PriceResponse,
  ArrayOfCoin,
} from './MarsOracleAdapter.types'
export interface MarsOracleAdapterMessage {
  contractAddress: string
  sender: string
  updateConfig: (
    {
      newConfig,
    }: {
      newConfig: ConfigUpdates
    },
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
}
export class MarsOracleAdapterMessageComposer implements MarsOracleAdapterMessage {
  sender: string
  contractAddress: string

  constructor(sender: string, contractAddress: string) {
    this.sender = sender
    this.contractAddress = contractAddress
    this.updateConfig = this.updateConfig.bind(this)
  }

  updateConfig = (
    {
      newConfig,
    }: {
      newConfig: ConfigUpdates
    },
    funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            update_config: {
              new_config: newConfig,
            },
          }),
        ),
        funds,
      }),
    }
  }
}
