// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { MsgExecuteContractEncodeObject } from 'cosmwasm'
import { MsgExecuteContract } from 'cosmjs-types/cosmwasm/wasm/v1/tx'
import { toUtf8 } from '@cosmjs/encoding'
import {
  HealthContractBaseForString,
  Uint128,
  OracleBaseForString,
  ParamsBaseForString,
  RedBankBaseForString,
  SwapperBaseForString,
  Decimal,
  ZapperBaseForString,
  InstantiateMsg,
  VaultInstantiateConfig,
  VaultConfig,
  Coin,
  VaultBaseForString,
  ExecuteMsg,
  Action,
  ActionAmount,
  LiquidateRequestForVaultBaseForString,
  VaultPositionType,
  EmergencyUpdate,
  OwnerUpdate,
  CallbackMsg,
  Addr,
  LiquidateRequestForVaultBaseForAddr,
  ActionCoin,
  ConfigUpdates,
  NftConfigUpdates,
  VaultBaseForAddr,
  QueryMsg,
  VaultPositionAmount,
  VaultAmount,
  VaultAmount1,
  UnlockingPositions,
  VaultPosition,
  LockingVaultAmount,
  VaultUnlockingPosition,
  ArrayOfCoinBalanceResponseItem,
  CoinBalanceResponseItem,
  ArrayOfSharesResponseItem,
  SharesResponseItem,
  ArrayOfDebtShares,
  DebtShares,
  ArrayOfLentShares,
  LentShares,
  ArrayOfVaultWithBalance,
  VaultWithBalance,
  ArrayOfVaultPositionResponseItem,
  VaultPositionResponseItem,
  ArrayOfString,
  ConfigResponse,
  OwnerResponse,
  ArrayOfCoin,
  Positions,
  DebtAmount,
  LentAmount,
  VaultConfigResponse,
  VaultPositionValue,
  CoinValue,
  VaultUtilizationResponse,
  ArrayOfVaultConfigResponse,
} from './MarsCreditManager.types'
export interface MarsCreditManagerMessage {
  contractAddress: string
  sender: string
  createCreditAccount: (funds?: Coin[]) => MsgExecuteContractEncodeObject
  updateCreditAccount: (
    {
      accountId,
      actions,
    }: {
      accountId: string
      actions: Action[]
    },
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  repayFromWallet: (
    {
      accountId,
    }: {
      accountId: string
    },
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  updateConfig: (
    {
      updates,
    }: {
      updates: ConfigUpdates
    },
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  emergencyConfigUpdate: (
    emergencyUpdate: EmergencyUpdate,
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  updateOwner: (ownerUpdate: OwnerUpdate, funds?: Coin[]) => MsgExecuteContractEncodeObject
  updateNftConfig: (
    {
      config,
      ownership,
    }: {
      config?: NftConfigUpdates
      ownership?: Action
    },
    funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  callback: (callbackMsg: CallbackMsg, funds?: Coin[]) => MsgExecuteContractEncodeObject
}
export class MarsCreditManagerMessageComposer implements MarsCreditManagerMessage {
  sender: string
  contractAddress: string

  constructor(sender: string, contractAddress: string) {
    this.sender = sender
    this.contractAddress = contractAddress
    this.createCreditAccount = this.createCreditAccount.bind(this)
    this.updateCreditAccount = this.updateCreditAccount.bind(this)
    this.repayFromWallet = this.repayFromWallet.bind(this)
    this.updateConfig = this.updateConfig.bind(this)
    this.emergencyConfigUpdate = this.emergencyConfigUpdate.bind(this)
    this.updateOwner = this.updateOwner.bind(this)
    this.updateNftConfig = this.updateNftConfig.bind(this)
    this.callback = this.callback.bind(this)
  }

  createCreditAccount = (funds?: Coin[]): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            create_credit_account: {},
          }),
        ),
        funds,
      }),
    }
  }
  updateCreditAccount = (
    {
      accountId,
      actions,
    }: {
      accountId: string
      actions: Action[]
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
            update_credit_account: {
              account_id: accountId,
              actions,
            },
          }),
        ),
        funds,
      }),
    }
  }
  repayFromWallet = (
    {
      accountId,
    }: {
      accountId: string
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
            repay_from_wallet: {
              account_id: accountId,
            },
          }),
        ),
        funds,
      }),
    }
  }
  updateConfig = (
    {
      updates,
    }: {
      updates: ConfigUpdates
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
              updates,
            },
          }),
        ),
        funds,
      }),
    }
  }
  emergencyConfigUpdate = (
    emergencyUpdate: EmergencyUpdate,
    funds?: Coin[],
  ): MsgExecuteContractEncodeObject => {
    return {
      typeUrl: '/cosmwasm.wasm.v1.MsgExecuteContract',
      value: MsgExecuteContract.fromPartial({
        sender: this.sender,
        contract: this.contractAddress,
        msg: toUtf8(
          JSON.stringify({
            emergency_config_update: emergencyUpdate,
          }),
        ),
        funds,
      }),
    }
  }
  updateOwner = (ownerUpdate: OwnerUpdate, funds?: Coin[]): MsgExecuteContractEncodeObject => {
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
        funds,
      }),
    }
  }
  updateNftConfig = (
    {
      config,
      ownership,
    }: {
      config?: NftConfigUpdates
      ownership?: Action
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
            update_nft_config: {
              config,
              ownership,
            },
          }),
        ),
        funds,
      }),
    }
  }
  callback = (callbackMsg: CallbackMsg, funds?: Coin[]): MsgExecuteContractEncodeObject => {
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
        funds,
      }),
    }
  }
}
