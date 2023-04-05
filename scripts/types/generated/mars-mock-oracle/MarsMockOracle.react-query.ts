// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.27.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from '@tanstack/react-query'
import { ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee, Coin } from '@cosmjs/amino'
import {
  Decimal,
  InstantiateMsg,
  CoinPrice,
  ExecuteMsg,
  QueryMsg,
  PriceResponse,
} from './MarsMockOracle.types'
import { MarsMockOracleQueryClient, MarsMockOracleClient } from './MarsMockOracle.client'
export const marsMockOracleQueryKeys = {
  contract: [
    {
      contract: 'marsMockOracle',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...marsMockOracleQueryKeys.contract[0], address: contractAddress }] as const,
  price: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...marsMockOracleQueryKeys.address(contractAddress)[0], method: 'price', args }] as const,
}
export interface MarsMockOracleReactQuery<TResponse, TData = TResponse> {
  client: MarsMockOracleQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface MarsMockOraclePriceQuery<TData>
  extends MarsMockOracleReactQuery<PriceResponse, TData> {
  args: {
    denom: string
  }
}
export function useMarsMockOraclePriceQuery<TData = PriceResponse>({
  client,
  args,
  options,
}: MarsMockOraclePriceQuery<TData>) {
  return useQuery<PriceResponse, Error, TData>(
    marsMockOracleQueryKeys.price(client?.contractAddress, args),
    () =>
      client
        ? client.price({
            denom: args.denom,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsMockOracleChangePriceMutation {
  client: MarsMockOracleClient
  msg: {
    denom: string
    price: Decimal
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsMockOracleChangePriceMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsMockOracleChangePriceMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsMockOracleChangePriceMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.changePrice(msg, fee, memo, funds),
    options,
  )
}
