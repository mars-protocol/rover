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
  Decimal,
  InstantiateMsg,
  ExecuteMsg,
  OwnerUpdate,
  AssetParamsUpdate,
  Uint128,
  VaultConfigUpdate,
  EmergencyUpdate,
  RoverEmergencyUpdate,
  RedBankEmergencyUpdate,
  AssetParams,
  RedBankSettings,
  RoverSettings,
  HighLeverageStrategyParams,
  VaultConfigBaseForString,
  Coin,
  QueryMsg,
  ArrayOfAssetParams,
  Addr,
  ArrayOfVaultConfigBaseForAddr,
  VaultConfigBaseForAddr,
  OwnerResponse,
} from './MarsParams.types'
export interface MarsParamsMessage {
  contractAddress: string
  sender: string
  updateOwner: (ownerUpdate: OwnerUpdate, _funds?: Coin[]) => MsgExecuteContractEncodeObject
  updateMaxCloseFactor: (_funds?: Coin[]) => MsgExecuteContractEncodeObject
  updateAssetParams: (
    assetParamsUpdate: AssetParamsUpdate,
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  updateVaultConfig: (
    vaultConfigUpdate: VaultConfigUpdate,
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  emergencyUpdate: (
    emergencyUpdate: EmergencyUpdate,
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
}
export class MarsParamsMessageComposer implements MarsParamsMessage {
  sender: string
  contractAddress: string

  constructor(sender: string, contractAddress: string) {
    this.sender = sender
    this.contractAddress = contractAddress
    this.updateOwner = this.updateOwner.bind(this)
    this.updateMaxCloseFactor = this.updateMaxCloseFactor.bind(this)
    this.updateAssetParams = this.updateAssetParams.bind(this)
    this.updateVaultConfig = this.updateVaultConfig.bind(this)
    this.emergencyUpdate = this.emergencyUpdate.bind(this)
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
  updateMaxCloseFactor = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            update_max_close_factor: {},
          }),
        ),
        funds: _funds,
      }),
    }
  }
  updateAssetParams = (
    assetParamsUpdate: AssetParamsUpdate,
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            update_asset_params: assetParamsUpdate,
          }),
        ),
        funds: _funds,
      }),
    }
  }
  updateVaultConfig = (
    vaultConfigUpdate: VaultConfigUpdate,
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            update_vault_config: vaultConfigUpdate,
          }),
        ),
        funds: _funds,
      }),
    }
  }
  emergencyUpdate = (
    emergencyUpdate: EmergencyUpdate,
    _funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            emergency_update: emergencyUpdate,
          }),
        ),
        funds: _funds,
      }),
    }
  }
}
