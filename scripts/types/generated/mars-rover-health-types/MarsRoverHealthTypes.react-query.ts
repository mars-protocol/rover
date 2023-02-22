// @ts-nocheck
/**
 * This file was automatically generated by @cosmwasm/ts-codegen@0.24.0.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run the @cosmwasm/ts-codegen generate command to regenerate this file.
 */

import { UseQueryOptions, useQuery, useMutation, UseMutationOptions } from '@tanstack/react-query'
import { ExecuteResult } from '@cosmjs/cosmwasm-stargate'
import { StdFee, Coin } from '@cosmjs/amino'
import {
  InstantiateMsg,
  ExecuteMsg,
  OwnerUpdate,
  QueryMsg,
  ConfigResponse,
  OwnerResponse,
  Decimal,
  Uint128,
  HealthResponse,
} from './MarsRoverHealthTypes.types'
import {
  MarsRoverHealthTypesQueryClient,
  MarsRoverHealthTypesClient,
} from './MarsRoverHealthTypes.client'
export const marsRoverHealthTypesQueryKeys = {
  contract: [
    {
      contract: 'marsRoverHealthTypes',
    },
  ] as const,
  address: (contractAddress: string | undefined) =>
    [{ ...marsRoverHealthTypesQueryKeys.contract[0], address: contractAddress }] as const,
  health: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsRoverHealthTypesQueryKeys.address(contractAddress)[0], method: 'health', args },
    ] as const,
  config: (contractAddress: string | undefined, args?: Record<string, unknown>) =>
    [
      { ...marsRoverHealthTypesQueryKeys.address(contractAddress)[0], method: 'config', args },
    ] as const,
}
export interface MarsRoverHealthTypesReactQuery<TResponse, TData = TResponse> {
  client: MarsRoverHealthTypesQueryClient | undefined
  options?: Omit<
    UseQueryOptions<TResponse, Error, TData>,
    "'queryKey' | 'queryFn' | 'initialData'"
  > & {
    initialData?: undefined
  }
}
export interface MarsRoverHealthTypesConfigQuery<TData>
  extends MarsRoverHealthTypesReactQuery<ConfigResponse, TData> {}
export function useMarsRoverHealthTypesConfigQuery<TData = ConfigResponse>({
  client,
  options,
}: MarsRoverHealthTypesConfigQuery<TData>) {
  return useQuery<ConfigResponse, Error, TData>(
    marsRoverHealthTypesQueryKeys.config(client?.contractAddress),
    () => (client ? client.config() : Promise.reject(new Error('Invalid client'))),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsRoverHealthTypesHealthQuery<TData>
  extends MarsRoverHealthTypesReactQuery<HealthResponse, TData> {
  args: {
    accountId: string
  }
}
export function useMarsRoverHealthTypesHealthQuery<TData = HealthResponse>({
  client,
  args,
  options,
}: MarsRoverHealthTypesHealthQuery<TData>) {
  return useQuery<HealthResponse, Error, TData>(
    marsRoverHealthTypesQueryKeys.health(client?.contractAddress, args),
    () =>
      client
        ? client.health({
            accountId: args.accountId,
          })
        : Promise.reject(new Error('Invalid client')),
    { ...options, enabled: !!client && (options?.enabled != undefined ? options.enabled : true) },
  )
}
export interface MarsRoverHealthTypesUpdateConfigMutation {
  client: MarsRoverHealthTypesClient
  msg: {
    creditManager: string
  }
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsRoverHealthTypesUpdateConfigMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsRoverHealthTypesUpdateConfigMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsRoverHealthTypesUpdateConfigMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) =>
      client.updateConfig(msg, fee, memo, funds),
    options,
  )
}
export interface MarsRoverHealthTypesUpdateOwnerMutation {
  client: MarsRoverHealthTypesClient
  args?: {
    fee?: number | StdFee | 'auto'
    memo?: string
    funds?: Coin[]
  }
}
export function useMarsRoverHealthTypesUpdateOwnerMutation(
  options?: Omit<
    UseMutationOptions<ExecuteResult, Error, MarsRoverHealthTypesUpdateOwnerMutation>,
    'mutationFn'
  >,
) {
  return useMutation<ExecuteResult, Error, MarsRoverHealthTypesUpdateOwnerMutation>(
    ({ client, msg, args: { fee, memo, funds } = {} }) => client.updateOwner(msg, fee, memo, funds),
    options,
  )
}