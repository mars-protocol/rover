// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
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
} from './MockOracle.types'
import { MockOracleQueryClient, MockOracleClient } from './MockOracle.client'
export const mockOracleQueryKeys = {
  contract: [
    {
      contract: 'mockOracle',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...mockOracleQueryKeys.contract[0], address: contractAddress }] as const,
  price: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [{ ...mockOracleQueryKeys.address(contractAddress)[0], method: 'price', args }] as const,
}
export interface MockOracleReactQuery<TResponse, TData = TResponse> {
  client: MockOracleQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface MockOraclePriceQuery<TData> extends MockOracleReactQuery<PriceResponse, TData> {
  args: {
    denom: string
  }
}
export function useMockOraclePriceQuery<TData = PriceResponse>({
  client,
  args,
  options,
}: MockOraclePriceQuery<TData>) {
  return useQuery<PriceResponse, Error, TData>(
    mockOracleQueryKeys.price(client?.contractAddress, args),
    () =>
      client
        ? client.price({
            denom: args.denom,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MockOracleChangePriceMutation {
  client: MockOracleClient
  msg: CoinPrice
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMockOracleChangePriceMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MockOracleChangePriceMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MockOracleChangePriceMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.changePrice(msg, fee, memo, funds),
    options,
  )
}
