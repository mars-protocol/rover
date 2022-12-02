// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from '@tanstack/react-query'
import { ExecuteResult } from '@cosmjs/cosmwasm-stargate'
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
import { MarsMockZapperQueryClient, MarsMockZapperClient } from './MarsMockZapper.client'
export const marsMockZapperQueryKeys = {
  contract: [
    {
      contract: 'marsMockZapper',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...marsMockZapperQueryKeys.contract[0], address: contractAddress }] as const,
  estimateProvideLiquidity: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      {
        ...marsMockZapperQueryKeys.address(contractAddress)[0],
        method: 'estimate_provide_liquidity',
        args,
      },
    ] as const,
  estimateWithdrawLiquidity: (
    contractAddress: string | undefined,
    args?: Record<string, unknown>,
  ) =>
    [
      {
        ...marsMockZapperQueryKeys.address(contractAddress)[0],
        method: 'estimate_withdraw_liquidity',
        args,
      },
    ] as const,
}
export interface MarsMockZapperReactQuery<TResponse, TData = TResponse> {
  client: MarsMockZapperQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface MarsMockZapperEstimateWithdrawLiquidityQuery<TData>
  extends MarsMockZapperReactQuery<ArrayOfCoin, TData> {
  args: {
    coinIn: Coin
  }
}
export function useMarsMockZapperEstimateWithdrawLiquidityQuery<TData = ArrayOfCoin>({
  client,
  args,
  options,
}: MarsMockZapperEstimateWithdrawLiquidityQuery<TData>) {
  return useQuery<ArrayOfCoin, Error, TData>(
    marsMockZapperQueryKeys.estimateWithdrawLiquidity(client?.contractAddress, args),
    () =>
      client
        ? client.estimateWithdrawLiquidity({
            coinIn: args.coinIn,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsMockZapperEstimateProvideLiquidityQuery<TData>
  extends MarsMockZapperReactQuery<Uint128, TData> {
  args: {
    coinsIn: Coin[]
    lpTokenOut: string
  }
}
export function useMarsMockZapperEstimateProvideLiquidityQuery<TData = Uint128>({
  client,
  args,
  options,
}: MarsMockZapperEstimateProvideLiquidityQuery<TData>) {
  return useQuery<Uint128, Error, TData>(
    marsMockZapperQueryKeys.estimateProvideLiquidity(client?.contractAddress, args),
    () =>
      client
        ? client.estimateProvideLiquidity({
            coinsIn: args.coinsIn,
            lpTokenOut: args.lpTokenOut,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsMockZapperWithdrawLiquidityMutation {
  client: MarsMockZapperClient
  msg: {
    recipient?: string
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsMockZapperWithdrawLiquidityMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsMockZapperWithdrawLiquidityMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsMockZapperWithdrawLiquidityMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.withdrawLiquidity(msg, fee, memo, funds),
    options,
  )
}
export interface MarsMockZapperProvideLiquidityMutation {
  client: MarsMockZapperClient
  msg: {
    lpTokenOut: string
    minimumReceive: Uint128
    recipient?: string
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsMockZapperProvideLiquidityMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsMockZapperProvideLiquidityMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsMockZapperProvideLiquidityMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.provideLiquidity(msg, fee, memo, funds),
    options,
  )
}
