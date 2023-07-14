import { DeploymentConfig, VaultType } from '../../types/config'

// Note: since osmo-test-5 upgrade, testnet and mainnet denoms are no longer the same. Reference asset info here: https://docs.osmosis.zone/osmosis-core/asset-info/
const uosmo = 'uosmo'
const ion = 'uion'
const aUSDC = 'ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858' // axelar USDC
const atom = 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2'

const atom_osmo = 'gamm/pool12'
const aUSDC_osmo = 'gamm/pool/5'
const ion_osmo = 'gamm/pool/1'

// All vaults below are ONE day vaults
const atom_osmo_vault = 'osmo1m45ap4rq4m2mfjkcqu9ks9mxmyx2hvx0cdca9sjmrg46q7lghzqqhxxup5'
const aUSDC_osmo_vault = 'osmo1l3q4mrhkzjyernjhg8lz2t52ddw589y5qc0z7y8y28h6y5wcl46sg9n28j'
const ion_osmo_vault = 'osmo1xwh9fqsla39v4px4qreztdegwy4czh4jepwgrfd94c03gphd0tjspfg86d'

const ATOM_OSMO_Config = (addr: string) => ({
  addr,
  deposit_cap: { denom: aUSDC, amount: '1000000000' }, // 1000 atom
  max_loan_to_value: '0.63',
  liquidation_threshold: '0.65',
  whitelisted: true,
})
const aUSDC_OSMO_Config = (addr: string) => ({
  addr,
  deposit_cap: { denom: aUSDC, amount: '1000000000' }, // 1000 atom
  max_loan_to_value: '0.63',
  liquidation_threshold: '0.65',
  whitelisted: true,
})

const ION_OSMO_Config = (addr: string) => ({
  addr,
  deposit_cap: { denom: aUSDC, amount: '1000000000' }, // 1000 atom
  max_loan_to_value: '0.63',
  liquidation_threshold: '0.65',
  whitelisted: true,
})

export const osmosisTestnetConfig: DeploymentConfig = {
  allowedCoins: [uosmo, aUSDC, ion, atom, ion_osmo, aUSDC_osmo, atom_osmo],
  chain: {
    baseDenom: uosmo,
    defaultGasPrice: 0.1,
    id: 'osmo-test-5',
    prefix: 'osmo',
    rpcEndpoint: 'https://rpc.osmotest5.osmosis.zone',
  },
  deployerMnemonic:
    'rely wonder join knock during sudden slow plate segment state agree also arrest mandate grief ordinary lonely lawsuit hurt super banana rule velvet cart',
  maxUnlockingPositions: '10',
  maxValueForBurn: '1000000',
  // Latest from: https://github.com/mars-protocol/outposts/blob/master/scripts/deploy/addresses/osmo-test-5.json
  oracle: { addr: 'osmo1m8kefut732j0slz9cv0wzycty5ndfd460yrhpkrqxfenrcz9wvnq5vd0hc' },
  redBank: { addr: 'osmo1lygdqcp400zzkwvsp4vqy96pfuxx2dwv7gus55v0vkvjs30fd0hsedetd9' },
  params: { addr: 'osmo1qkdre77sgeqyyt9e4w66d6faa77p8h63anmj4tj64lfcefd3zk9qc63wqs' },
  swapRoutes: [
    { denomIn: uosmo, denomOut: aUSDC, route: [{ token_out_denom: aUSDC, pool_id: aUSDC_osmo }] },
    { denomIn: aUSDC, denomOut: uosmo, route: [{ token_out_denom: uosmo, pool_id: aUSDC_osmo }] },
    { denomIn: uosmo, denomOut: ion, route: [{ token_out_denom: ion, pool_id: ion_osmo }] },
    { denomIn: ion, denomOut: uosmo, route: [{ token_out_denom: uosmo, pool_id: ion_osmo }] },
    { denomIn: uosmo, denomOut: atom, route: [{ token_out_denom: atom, pool_id: atom_osmo }] },
    { denomIn: atom, denomOut: uosmo, route: [{ token_out_denom: uosmo, pool_id: atom_osmo }] },
  ],
  // Latest from: https://api.apollo.farm/api/graph?query=query+MyQuery+%7B%0A++vaults%28network%3A+osmo_test_5%29+%7B%0A++++label%0A++++contract_address%0A++%7D%0A%7D
  vaults: [
    aUSDC_OSMO_Config(aUSDC_osmo_vault),
    ION_OSMO_Config(ion_osmo_vault),
    ATOM_OSMO_Config(atom_osmo_vault),
  ],
  swapperContractName: 'mars_swapper_osmosis',
  zapperContractName: 'mars_v2_zapper_osmosis',
  testActions: {
    allowedCoinsConfig: [
      { denom: uosmo, priceSource: { fixed: { price: '1' } }, grantCreditLine: true },
      {
        denom: aUSDC,
        priceSource: { geometric_twap: { pool_id: 5, window_size: 1800 } },
        grantCreditLine: true,
      },
      {
        denom: ion,
        priceSource: { geometric_twap: { pool_id: 1, window_size: 1800 } },
        grantCreditLine: true,
      },
      {
        denom: atom,
        priceSource: { geometric_twap: { pool_id: 12, window_size: 1800 } },
        grantCreditLine: true,
      },
      {
        denom: ion_osmo,
        priceSource: { xyk_liquidity_token: { pool_id: 1 } },
        grantCreditLine: false,
      },
      {
        denom: aUSDC_osmo,
        priceSource: { xyk_liquidity_token: { pool_id: 6 } },
        grantCreditLine: false,
      },
      {
        denom: atom_osmo,
        priceSource: { xyk_liquidity_token: { pool_id: 12 } },
        grantCreditLine: false,
      },
    ],
    vault: {
      depositAmount: '1000000',
      withdrawAmount: '1000000',
      mock: {
        config: {
          deposit_cap: { denom: aUSDC, amount: '100000000' }, // 100 usdc
          liquidation_threshold: '0.585',
          max_loan_to_value: '0.569',
          whitelisted: true,
        },
        vaultTokenDenom: uosmo,
        type: VaultType.LOCKED,
        lockup: { time: 900 }, // 15 mins
        baseToken: aUSDC_osmo,
      },
    },
    outpostsDeployerMnemonic:
      'elevator august inherit simple buddy giggle zone despair marine rich swim danger blur people hundred faint ladder wet toe strong blade utility trial process',
    borrowAmount: '10',
    repayAmount: '11',
    defaultCreditLine: '100000000000',
    depositAmount: '100',
    lendAmount: '10',
    reclaimAmount: '5',
    secondaryDenom: aUSDC,
    startingAmountForTestUser: '4000000',
    swap: {
      slippage: '0.4',
      amount: '40',
      route: [
        {
          token_out_denom: aUSDC,
          pool_id: '5',
        },
      ],
    },
    unzapAmount: '1000000',
    withdrawAmount: '12',
    zap: {
      coinsIn: [
        {
          denom: aUSDC,
          amount: '1',
        },
        { denom: uosmo, amount: '3' },
      ],
      denomOut: aUSDC_osmo,
    },
  },
}
