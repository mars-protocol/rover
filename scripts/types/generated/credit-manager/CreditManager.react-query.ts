// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from '@tanstack/react-query'
import { ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  Decimal,
  OracleBaseForString,
  RedBankBaseForString,
  SwapperBaseForString,
  InstantiateMsg,
  VaultBaseForString,
  ExecuteMsg,
  Action,
  Uint128,
  CallbackMsg,
  Addr,
  Coin,
  ConfigUpdates,
  VaultBaseForAddr,
  QueryMsg,
  ArrayOfCoinBalanceResponseItem,
  CoinBalanceResponseItem,
  ArrayOfSharesResponseItem,
  SharesResponseItem,
  ArrayOfDebtShares,
  DebtShares,
  ArrayOfVaultWithBalance,
  VaultWithBalance,
  ArrayOfVaultPositionResponseItem,
  VaultPositionResponseItem,
  VaultPosition,
  VaultPositionState,
  VaultUnlockingId,
  ArrayOfString,
  ArrayOfVaultBaseForString,
  ConfigResponse,
  HealthResponse,
  Positions,
  DebtAmount,
} from './CreditManager.types'
import { CreditManagerQueryClient, CreditManagerClient } from './CreditManager.client'
export const creditManagerQueryKeys = {
  contract: [
    {
      contract: 'creditManager',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...creditManagerQueryKeys.contract[0], address: contractAddress }] as const,
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...creditManagerQueryKeys.address(contractAddress)[0], method: 'config', args }] as const,
  allowedVaults: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...creditManagerQueryKeys.address(contractAddress)[0], method: 'allowed_vaults', args },
    ] as const,
  allowedCoins: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...creditManagerQueryKeys.address(contractAddress)[0], method: 'allowed_coins', args },
    ] as const,
  positions: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...creditManagerQueryKeys.address(contractAddress)[0], method: 'positions', args }] as const,
  health: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...creditManagerQueryKeys.address(contractAddress)[0], method: 'health', args }] as const,
  allCoinBalances: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...creditManagerQueryKeys.address(contractAddress)[0], method: 'all_coin_balances', args },
    ] as const,
  allDebtShares: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...creditManagerQueryKeys.address(contractAddress)[0], method: 'all_debt_shares', args },
    ] as const,
  totalDebtShares: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...creditManagerQueryKeys.address(contractAddress)[0], method: 'total_debt_shares', args },
    ] as const,
  allTotalDebtShares: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      {
        ...creditManagerQueryKeys.address(contractAddress)[0],
        method: 'all_total_debt_shares',
        args,
      },
    ] as const,
  allVaultPositions: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      {
        ...creditManagerQueryKeys.address(contractAddress)[0],
        method: 'all_vault_positions',
        args,
      },
    ] as const,
  totalVaultCoinBalance: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      {
        ...creditManagerQueryKeys.address(contractAddress)[0],
        method: 'total_vault_coin_balance',
        args,
      },
    ] as const,
  allTotalVaultCoinBalances: (
    contractAddress: string | undefined,
    args?: Record<string, unknown>,
  ) =>
    [
      {
        ...creditManagerQueryKeys.address(contractAddress)[0],
        method: 'all_total_vault_coin_balances',
        args,
      },
    ] as const,
}
export interface CreditManagerReactQuery<TResponse, TData = TResponse> {
  client: CreditManagerQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface CreditManagerAllTotalVaultCoinBalancesQuery<TData>
  extends CreditManagerReactQuery<ArrayOfVaultWithBalance, TData> {
  args: {
    limit?: number
    startAfter?: VaultBaseForString
  }
}
export function useCreditManagerAllTotalVaultCoinBalancesQuery<TData = ArrayOfVaultWithBalance>({
  client,
  args,
  options,
}: CreditManagerAllTotalVaultCoinBalancesQuery<TData>) {
  return useQuery<ArrayOfVaultWithBalance, Error, TData>(
    creditManagerQueryKeys.allTotalVaultCoinBalances(client?.contractAddress, args),
    () =>
      client
        ? client.allTotalVaultCoinBalances({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerTotalVaultCoinBalanceQuery<TData>
  extends CreditManagerReactQuery<Uint128, TData> {
  args: {
    vault: VaultBaseForString
  }
}
export function useCreditManagerTotalVaultCoinBalanceQuery<TData = Uint128>({
  client,
  args,
  options,
}: CreditManagerTotalVaultCoinBalanceQuery<TData>) {
  return useQuery<Uint128, Error, TData>(
    creditManagerQueryKeys.totalVaultCoinBalance(client?.contractAddress, args),
    () =>
      client
        ? client.totalVaultCoinBalance({
            vault: args.vault,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerAllVaultPositionsQuery<TData>
  extends CreditManagerReactQuery<ArrayOfVaultPositionResponseItem, TData> {
  args: {
    limit?: number
    startAfter?: string[][]
  }
}
export function useCreditManagerAllVaultPositionsQuery<TData = ArrayOfVaultPositionResponseItem>({
  client,
  args,
  options,
}: CreditManagerAllVaultPositionsQuery<TData>) {
  return useQuery<ArrayOfVaultPositionResponseItem, Error, TData>(
    creditManagerQueryKeys.allVaultPositions(client?.contractAddress, args),
    () =>
      client
        ? client.allVaultPositions({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerAllTotalDebtSharesQuery<TData>
  extends CreditManagerReactQuery<ArrayOfDebtShares, TData> {
  args: {
    limit?: number
    startAfter?: string
  }
}
export function useCreditManagerAllTotalDebtSharesQuery<TData = ArrayOfDebtShares>({
  client,
  args,
  options,
}: CreditManagerAllTotalDebtSharesQuery<TData>) {
  return useQuery<ArrayOfDebtShares, Error, TData>(
    creditManagerQueryKeys.allTotalDebtShares(client?.contractAddress, args),
    () =>
      client
        ? client.allTotalDebtShares({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerTotalDebtSharesQuery<TData>
  extends CreditManagerReactQuery<DebtShares, TData> {}
export function useCreditManagerTotalDebtSharesQuery<TData = DebtShares>({
  client,
  options,
}: CreditManagerTotalDebtSharesQuery<TData>) {
  return useQuery<DebtShares, Error, TData>(
    creditManagerQueryKeys.totalDebtShares(client?.contractAddress),
    () => (client ? client.totalDebtShares() : Promise.reject(new Error('Invalid client'))),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerAllDebtSharesQuery<TData>
  extends CreditManagerReactQuery<ArrayOfSharesResponseItem, TData> {
  args: {
    limit?: number
    startAfter?: string[][]
  }
}
export function useCreditManagerAllDebtSharesQuery<TData = ArrayOfSharesResponseItem>({
  client,
  args,
  options,
}: CreditManagerAllDebtSharesQuery<TData>) {
  return useQuery<ArrayOfSharesResponseItem, Error, TData>(
    creditManagerQueryKeys.allDebtShares(client?.contractAddress, args),
    () =>
      client
        ? client.allDebtShares({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerAllCoinBalancesQuery<TData>
  extends CreditManagerReactQuery<ArrayOfCoinBalanceResponseItem, TData> {
  args: {
    limit?: number
    startAfter?: string[][]
  }
}
export function useCreditManagerAllCoinBalancesQuery<TData = ArrayOfCoinBalanceResponseItem>({
  client,
  args,
  options,
}: CreditManagerAllCoinBalancesQuery<TData>) {
  return useQuery<ArrayOfCoinBalanceResponseItem, Error, TData>(
    creditManagerQueryKeys.allCoinBalances(client?.contractAddress, args),
    () =>
      client
        ? client.allCoinBalances({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerHealthQuery<TData>
  extends CreditManagerReactQuery<HealthResponse, TData> {
  args: {
    accountId: string
  }
}
export function useCreditManagerHealthQuery<TData = HealthResponse>({
  client,
  args,
  options,
}: CreditManagerHealthQuery<TData>) {
  return useQuery<HealthResponse, Error, TData>(
    creditManagerQueryKeys.health(client?.contractAddress, args),
    () =>
      client
        ? client.health({
            accountId: args.accountId,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerPositionsQuery<TData>
  extends CreditManagerReactQuery<Positions, TData> {
  args: {
    accountId: string
  }
}
export function useCreditManagerPositionsQuery<TData = Positions>({
  client,
  args,
  options,
}: CreditManagerPositionsQuery<TData>) {
  return useQuery<Positions, Error, TData>(
    creditManagerQueryKeys.positions(client?.contractAddress, args),
    () =>
      client
        ? client.positions({
            accountId: args.accountId,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerAllowedCoinsQuery<TData>
  extends CreditManagerReactQuery<ArrayOfString, TData> {
  args: {
    limit?: number
    startAfter?: string
  }
}
export function useCreditManagerAllowedCoinsQuery<TData = ArrayOfString>({
  client,
  args,
  options,
}: CreditManagerAllowedCoinsQuery<TData>) {
  return useQuery<ArrayOfString, Error, TData>(
    creditManagerQueryKeys.allowedCoins(client?.contractAddress, args),
    () =>
      client
        ? client.allowedCoins({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerAllowedVaultsQuery<TData>
  extends CreditManagerReactQuery<ArrayOfVaultBaseForString, TData> {
  args: {
    limit?: number
    startAfter?: VaultBaseForString
  }
}
export function useCreditManagerAllowedVaultsQuery<TData = ArrayOfVaultBaseForString>({
  client,
  args,
  options,
}: CreditManagerAllowedVaultsQuery<TData>) {
  return useQuery<ArrayOfVaultBaseForString, Error, TData>(
    creditManagerQueryKeys.allowedVaults(client?.contractAddress, args),
    () =>
      client
        ? client.allowedVaults({
            limit: args.limit,
            startAfter: args.startAfter,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerConfigQuery<TData>
  extends CreditManagerReactQuery<ConfigResponse, TData> {}
export function useCreditManagerConfigQuery<TData = ConfigResponse>({
  client,
  options,
}: CreditManagerConfigQuery<TData>) {
  return useQuery<ConfigResponse, Error, TData>(
    creditManagerQueryKeys.config(client?.contractAddress),
    () => (client ? client.config() : Promise.reject(new Error('Invalid client'))),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface CreditManagerCallbackMutation {
  client: CreditManagerClient
  msg: CallbackMsg
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useCreditManagerCallbackMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, CreditManagerCallbackMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, CreditManagerCallbackMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.callback(msg, fee, memo, funds),
    options,
  )
}
export interface CreditManagerUpdateConfigMutation {
  client: CreditManagerClient
  msg: {
    newConfig: ConfigUpdates
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useCreditManagerUpdateConfigMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, CreditManagerUpdateConfigMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, CreditManagerUpdateConfigMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.updateConfig(msg, fee, memo, funds),
    options,
  )
}
export interface CreditManagerUpdateCreditAccountMutation {
  client: CreditManagerClient
  msg: {
    accountId: string
    actions: Action[]
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useCreditManagerUpdateCreditAccountMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, CreditManagerUpdateCreditAccountMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, CreditManagerUpdateCreditAccountMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.updateCreditAccount(msg, fee, memo, funds),
    options,
  )
}
export interface CreditManagerCreateCreditAccountMutation {
  client: CreditManagerClient
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useCreditManagerCreateCreditAccountMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, CreditManagerCreateCreditAccountMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, CreditManagerCreateCreditAccountMutation>(
    ({ client, args: { fee, memo, funds } = {} }) => client.createCreditAccount(fee, memo, funds),
    options,
  )
}
