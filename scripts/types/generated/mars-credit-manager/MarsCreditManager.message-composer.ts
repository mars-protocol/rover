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
  HealthContractBaseForString,
  Uint128,
  OracleBaseForString,
  ParamsBaseForString,
  RedBankBaseForString,
  SwapperBaseForString,
  ZapperBaseForString,
  InstantiateMsg,
  ExecuteMsg,
  AccountKind,
  Action,
  ActionAmount,
  LiquidateRequestForVaultBaseForString,
  VaultPositionType,
  Decimal,
  AccountNftBaseForString,
  OwnerUpdate,
  CallbackMsg,
  Addr,
  HealthState,
  LiquidateRequestForVaultBaseForAddr,
  ChangeExpected,
  Coin,
  ActionCoin,
  VaultBaseForString,
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
  ArrayOfVaultPositionResponseItem,
  VaultPositionResponseItem,
  ConfigResponse,
  OwnerResponse,
  ArrayOfCoin,
  Positions,
  DebtAmount,
  LentAmount,
  VaultPositionValue,
  CoinValue,
  VaultUtilizationResponse,
} from './MarsCreditManager.types'
export interface MarsCreditManagerMessage {
  contractAddress: string
  sender: string
  createCreditAccount: (_funds?: Coin[]) => MsgExecuteContractEncodeObject
  updateCreditAccount: (
    {
      accountId,
      actions,
    }: {
      accountId: string
      actions: Action[]
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  repayFromWallet: (
    {
      accountId,
    }: {
      accountId: string
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  updateConfig: (
    {
      updates,
    }: {
      updates: ConfigUpdates
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  updateOwner: (ownerUpdate: OwnerUpdate, _funds?: Coin[]) => MsgExecuteContractEncodeObject
  updateNftConfig: (
    {
      config,
      ownership,
    }: {
      config?: NftConfigUpdates
      ownership?: Action
    },
    _funds?: Coin[],
  ) => MsgExecuteContractEncodeObject
  callback: (callbackMsg: CallbackMsg, _funds?: Coin[]) => MsgExecuteContractEncodeObject
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
    this.updateOwner = this.updateOwner.bind(this)
    this.updateNftConfig = this.updateNftConfig.bind(this)
    this.callback = this.callback.bind(this)
  }

  createCreditAccount = (_funds?: Coin[]): MsgExecuteContractEncodeObject => {
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
        funds: _funds,
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
    _funds?: Coin[],
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
        funds: _funds,
      }),
    }
  }
  repayFromWallet = (
    {
      accountId,
    }: {
      accountId: string
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
            repay_from_wallet: {
              account_id: accountId,
            },
          }),
        ),
        funds: _funds,
      }),
    }
  }
  updateConfig = (
    {
      updates,
    }: {
      updates: ConfigUpdates
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
            update_config: {
              updates,
            },
          }),
        ),
        funds: _funds,
      }),
    }
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
  updateNftConfig = (
    {
      config,
      ownership,
    }: {
      config?: NftConfigUpdates
      ownership?: Action
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
            update_nft_config: {
              config,
              ownership,
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
