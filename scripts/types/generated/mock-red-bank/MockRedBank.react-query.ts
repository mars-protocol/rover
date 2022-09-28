// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from '@tanstack/react-query'
import { ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee } from '@cosmjs/amino'
import {
  Decimal,
  InstantiateMsg,
  CoinMarketInfo,
  ExecuteMsg,
  Uint128,
  Coin,
  QueryMsg,
  Market,
  InterestRateModel,
  UserAssetDebtResponse,
} from './MockRedBank.types'
import { MockRedBankQueryClient, MockRedBankClient } from './MockRedBank.client'
export const mockRedBankQueryKeys = {
  contract: [
    {
      contract: 'mockRedBank',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...mockRedBankQueryKeys.contract[0], address: contractAddress }] as const,
  userAssetDebt: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...mockRedBankQueryKeys.address(contractAddress)[0], method: 'user_asset_debt', args },
    ] as const,
  market: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...mockRedBankQueryKeys.address(contractAddress)[0], method: 'market', args }] as const,
}
export interface MockRedBankReactQuery<TResponse, TData = TResponse> {
  client: MockRedBankQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface MockRedBankMarketQuery<TData> extends MockRedBankReactQuery<Market, TData> {
  args: {
    denom: string
  }
}
export function useMockRedBankMarketQuery<TData = Market>({
  client,
  args,
  options,
}: MockRedBankMarketQuery<TData>) {
  return useQuery<Market, Error, TData>(
    mockRedBankQueryKeys.market(client?.contractAddress, args),
    () =>
      client
        ? client.market({
            denom: args.denom,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MockRedBankUserAssetDebtQuery<TData>
  extends MockRedBankReactQuery<UserAssetDebtResponse, TData> {
  args: {
    denom: string
    userAddress: string
  }
}
export function useMockRedBankUserAssetDebtQuery<TData = UserAssetDebtResponse>({
  client,
  args,
  options,
}: MockRedBankUserAssetDebtQuery<TData>) {
  return useQuery<UserAssetDebtResponse, Error, TData>(
    mockRedBankQueryKeys.userAssetDebt(client?.contractAddress, args),
    () =>
      client
        ? client.userAssetDebt({
            denom: args.denom,
            userAddress: args.userAddress,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MockRedBankRepayMutation {
  client: MockRedBankClient
  msg: {
    denom: string
    onBehalfOf?: string
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMockRedBankRepayMutation(
  options?: Omit<UseMutationOptions<ExecuteResult, Error, MockRedBankRepayMutation>, 'mutationFn'>,
) {
  return useMutation<ExecuteResult, Error, MockRedBankRepayMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.repay(msg, fee, memo, funds),
    options,
  )
}
export interface MockRedBankBorrowMutation {
  client: MockRedBankClient
  msg: {
    coin: Coin
    recipient?: string
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMockRedBankBorrowMutation(
  options?: Omit<UseMutationOptions<ExecuteResult, Error, MockRedBankBorrowMutation>, 'mutationFn'>,
) {
  return useMutation<ExecuteResult, Error, MockRedBankBorrowMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.borrow(msg, fee, memo, funds),
    options,
  )
}
