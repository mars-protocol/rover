import { Positions } from '../types/generated/mars-credit-manager/MarsCreditManager.types'
import { MarsCreditManagerQueryClient } from '../types/generated/mars-credit-manager/MarsCreditManager.client'
import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate/build/cosmwasmclient'
import {
  AccountKind,
  HealthValuesResponse,
} from '../types/generated/mars-rover-health-types/MarsRoverHealthTypes.types'
import {
  DenomsData,
  HealthComputer,
  VaultsData,
} from '../types/generated/mars-rover-health-computer/MarsRoverHealthComputer.types'
import { MarsMockOracleQueryClient } from '../types/generated/mars-mock-oracle/MarsMockOracle.client'
import { MarsMockVaultQueryClient } from '../types/generated/mars-mock-vault/MarsMockVault.client'
import { MarsParamsQueryClient } from '../types/generated/mars-params/MarsParams.client'
import {
  BorrowTarget,
  compute_health_js,
  max_borrow_estimate_js,
  max_withdraw_estimate_js,
} from './pkg-web'

export class DataFetcher {
  constructor(
    private computeHealthFn: typeof compute_health_js,
    private maxWithdrawFn: typeof max_withdraw_estimate_js,
    private maxBorrowFn: typeof max_borrow_estimate_js,
    private creditManagerAddr: string,
    private oracleAddr: string,
    private paramsAddr: string,
    private rpcEndpoint: string,
  ) {}

  getClient = async (): Promise<CosmWasmClient> => {
    return await CosmWasmClient.connect(this.rpcEndpoint)
  }

  fetchPositions = async (accountId: string): Promise<Positions> => {
    const cmQuery = new MarsCreditManagerQueryClient(await this.getClient(), this.creditManagerAddr)
    return await cmQuery.positions({ accountId })
  }

  fetchParams = async (denoms: string[]): Promise<DenomsData['params']> => {
    const pQuery = new MarsParamsQueryClient(await this.getClient(), this.paramsAddr)
    const promises = denoms.map(async (denom) => ({
      denom: denom,
      params: await pQuery.assetParams({ denom }),
    }))
    const responses = await Promise.all(promises)
    return responses.reduce(
      (acc, curr) => {
        acc[curr.denom] = curr.params
        return acc
      },
      {} as DenomsData['params'],
    )
  }

  fetchPrices = async (denoms: string[]): Promise<DenomsData['prices']> => {
    const oQuery = new MarsMockOracleQueryClient(await this.getClient(), this.oracleAddr)
    const promises = denoms.map(async (denom) => await oQuery.price({ denom }))
    const responses = await Promise.all(promises)
    return responses.reduce(
      (acc, curr) => {
        acc[curr.denom] = curr.price
        return acc
      },
      {} as DenomsData['prices'],
    )
  }

  fetchDenomsData = async (positions: Positions): Promise<DenomsData> => {
    const depositDenoms = positions.deposits.map((c) => c.denom)
    const debtDenoms = positions.debts.map((c) => c.denom)
    const vaultBaseTokenDenoms = await Promise.all(
      positions.vaults.map(async (v) => {
        const vQuery = new MarsMockVaultQueryClient(await this.getClient(), v.vault.address)
        const info = await vQuery.info()
        return info.base_token
      }),
    )

    const allDenoms = depositDenoms.concat(debtDenoms).concat(vaultBaseTokenDenoms)

    return {
      params: await this.fetchParams(allDenoms),
      prices: await this.fetchPrices(allDenoms),
    }
  }

  fetchVaultsData = async (positions: Positions): Promise<VaultsData> => {
    const vaultsData = { vault_values: {}, vault_configs: {} } as VaultsData
    const cmQuery = new MarsCreditManagerQueryClient(await this.getClient(), this.creditManagerAddr)
    const pQuery = new MarsParamsQueryClient(await this.getClient(), this.paramsAddr)
    await Promise.all(
      positions.vaults.map(async (v) => {
        vaultsData.vault_values[v.vault.address] = await cmQuery.vaultPositionValue({
          vaultPosition: v,
        })

        vaultsData.vault_configs[v.vault.address] = await pQuery.vaultConfig({
          address: v.vault.address,
        })
      }),
    )
    return vaultsData
  }

  assembleComputer = async (accountId: string): Promise<HealthComputer> => {
    const positions = await this.fetchPositions(accountId)

    const [denoms_data, vaults_data] = await Promise.all([
      this.fetchDenomsData(positions),
      this.fetchVaultsData(positions),
    ])

    return {
      positions,
      denoms_data,
      vaults_data,
      kind: 'default' as AccountKind,
    }
  }

  computeHealth = async (accountId: string): Promise<HealthValuesResponse> => {
    // const positions = await this.assembleComputer(accountId)
    const positions: HealthComputer = {
      denoms_data: {
        params: {
          'gamm/pool/12': {
            denom: 'gamm/pool/12',
            credit_manager: {
              whitelisted: true,
              hls: null,
            },
            red_bank: {
              deposit_enabled: false,
              borrow_enabled: false,
            },
            max_loan_to_value: '0.68',
            liquidation_threshold: '0.7',
            liquidation_bonus: {
              starting_lb: '0',
              slope: '2',
              min_lb: '0',
              max_lb: '0.05',
            },
            protocol_liquidation_fee: '0.5',
            deposit_cap: '100000000000',
          },
          'gamm/pool/5': {
            denom: 'gamm/pool/5',
            credit_manager: {
              whitelisted: true,
              hls: null,
            },
            red_bank: {
              deposit_enabled: false,
              borrow_enabled: false,
            },
            max_loan_to_value: '0.68',
            liquidation_threshold: '0.7',
            liquidation_bonus: {
              starting_lb: '0',
              slope: '2',
              min_lb: '0',
              max_lb: '0.05',
            },
            protocol_liquidation_fee: '0.5',
            deposit_cap: '100000000000',
          },
          'ibc/6F34E1BD664C36CE49ACC28E60D62559A5F96C4F9A6CCE4FC5A67B2852E24CFE': {
            denom: 'ibc/6F34E1BD664C36CE49ACC28E60D62559A5F96C4F9A6CCE4FC5A67B2852E24CFE',
            credit_manager: {
              whitelisted: true,
              hls: null,
            },
            red_bank: {
              deposit_enabled: true,
              borrow_enabled: true,
            },
            max_loan_to_value: '0.74',
            liquidation_threshold: '0.75',
            liquidation_bonus: {
              starting_lb: '0',
              slope: '2',
              min_lb: '0',
              max_lb: '0.05',
            },
            protocol_liquidation_fee: '0.5',
            deposit_cap: '500000000000',
          },
          'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477': {
            denom: 'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477',
            credit_manager: {
              whitelisted: true,
              hls: null,
            },
            red_bank: {
              deposit_enabled: true,
              borrow_enabled: true,
            },
            max_loan_to_value: '0.68',
            liquidation_threshold: '0.7',
            liquidation_bonus: {
              starting_lb: '0',
              slope: '2',
              min_lb: '0',
              max_lb: '0.05',
            },
            protocol_liquidation_fee: '0.5',
            deposit_cap: '100000000000',
          },
          uosmo: {
            denom: 'uosmo',
            credit_manager: {
              whitelisted: true,
              hls: null,
            },
            red_bank: {
              deposit_enabled: true,
              borrow_enabled: true,
            },
            max_loan_to_value: '0.59',
            liquidation_threshold: '0.61',
            liquidation_bonus: {
              starting_lb: '0',
              slope: '2',
              min_lb: '0',
              max_lb: '0.05',
            },
            protocol_liquidation_fee: '0.5',
            deposit_cap: '2500000000000',
          },
        },
        prices: {
          uosmo: '1',
          'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477':
            '18.032601591187270502',
          'ibc/6F34E1BD664C36CE49ACC28E60D62559A5F96C4F9A6CCE4FC5A67B2852E24CFE':
            '2.44810249694002448',
          'ibc/DB9D326CF53EA07610C394D714D78F8BB4DC7E312D4213193791A9046BF45E20':
            '0.12466728549597313',
          usd: '2.447980416156670747',
        },
      },
      vaults_data: {
        vault_configs: {
          osmo1m45ap4rq4m2mfjkcqu9ks9mxmyx2hvx0cdca9sjmrg46q7lghzqqhxxup5: {
            addr: 'osmo1m45ap4rq4m2mfjkcqu9ks9mxmyx2hvx0cdca9sjmrg46q7lghzqqhxxup5',
            deposit_cap: {
              denom: 'ibc/6F34E1BD664C36CE49ACC28E60D62559A5F96C4F9A6CCE4FC5A67B2852E24CFE',
              amount: '1000000000',
            },
            max_loan_to_value: '0.63',
            liquidation_threshold: '0.65',
            whitelisted: true,
            hls: null,
          },
        },
        vault_values: {
          osmo1m45ap4rq4m2mfjkcqu9ks9mxmyx2hvx0cdca9sjmrg46q7lghzqqhxxup5: {
            base_coin: {
              amount: '0',
              denom: 'gamm/pool/12',
              value: '0',
            },
            vault_coin: {
              amount: '0',
              denom:
                'factory/osmo1m45ap4rq4m2mfjkcqu9ks9mxmyx2hvx0cdca9sjmrg46q7lghzqqhxxup5/cwVTT',
              value: '1930432',
            },
          },
        },
      },
      positions: {
        account_id: '10',
        debts: [
          {
            shares: '0',
            amount: '1000455',
            denom: 'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477',
          },
          {
            shares: '0',
            amount: '121681959',
            denom: 'uosmo',
          },
        ],
        deposits: [
          {
            denom: 'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477',
            amount: '947986',
          },
          {
            denom: 'uosmo',
            amount: '144046916',
          },
        ],
        lends: [
          {
            shares: '0',
            amount: '63792919',
            denom: 'ibc/6F34E1BD664C36CE49ACC28E60D62559A5F96C4F9A6CCE4FC5A67B2852E24CFE',
          },
          {
            shares: '0',
            amount: '13283922',
            denom: 'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477',
          },
          {
            shares: '0',
            amount: '267835023',
            denom: 'uosmo',
          },
        ],
        vaults: [
          {
            vault: {
              address: 'osmo1m45ap4rq4m2mfjkcqu9ks9mxmyx2hvx0cdca9sjmrg46q7lghzqqhxxup5',
            },
            amount: {
              locking: {
                locked: '55679789900460000000',
                unlocking: [
                  {
                    id: 0,
                    coin: {
                      amount: '0',
                      denom: 'gamm/pool/12',
                    },
                  },
                ],
              },
            },
          },
        ],
      },
      kind: 'default',
    }
    console.log('adjusted positions', positions)
    try {
      const res = this.computeHealthFn(positions)
      console.log('res', res)
    } catch (err) {
      console.error('err ', err)
    }
  }

  maxWithdrawAmount = async (accountId: string, denom: string): Promise<number> => {
    const positions = await this.assembleComputer(accountId)
    const result = this.maxWithdrawFn(positions, denom)
    return parseInt(result)
  }

  maxBorrowAmount = async (
    accountId: string,
    denom: string,
    target: BorrowTarget,
  ): Promise<number> => {
    const positions = await this.assembleComputer(accountId)
    const result = this.maxBorrowFn(positions, denom, target)
    return parseInt(result)
  }
}
