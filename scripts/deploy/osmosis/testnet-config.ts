import { DeploymentConfig, VaultType } from '../../types/config'

// Note: since osmo-test-5 upgrade, testnet and mainnet denoms are no longer the same. Reference asset info here: https://docs.osmosis.zone/osmosis-core/asset-info/
const uosmo = 'uosmo'
const aUSDC = 'ibc/6F34E1BD664C36CE49ACC28E60D62559A5F96C4F9A6CCE4FC5A67B2852E24CFE' // axelar USDC
const atom = 'ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477'

const ausdcOsmoPool = 'gamm/pool/5'
const atomOsmoPool = 'gamm/pool/12'

// All vaults below are ONE day vaults
const atomOsmoVault = 'osmo1m45ap4rq4m2mfjkcqu9ks9mxmyx2hvx0cdca9sjmrg46q7lghzqqhxxup5'
const ausdcOsmoVault = 'osmo1l3q4mrhkzjyernjhg8lz2t52ddw589y5qc0z7y8y28h6y5wcl46sg9n28j'

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

export const osmosisTestnetConfig: DeploymentConfig = {
  allowedCoins: [uosmo, aUSDC, atom, ausdcOsmoPool, atomOsmoPool],
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
  redBank: { addr: 'osmo132gavhwc4j5vvwsdcl80k6eywa3f6eln0lx7r7zy2xu288k98stsregmww' },
  incentives: { addr: 'osmo1zrnwn32vmky36yfpqym8uw4aj5z3g8rwu0zrrtplcu8t409enujqenwqnc' },
  oracle: { addr: 'osmo1mzfu9c4fzgtn2wcv2jkzytxgkwnpntke4jf6jg98rpcjenvqxe9s3k6duh' },
  swapper: { addr: 'osmo1xc7zuag82enh8lqhrd48s9aesu28pvkeh06rg9ns6v7lwerw3y6shr0nyh' },
  params: { addr: 'osmo1xn978f8cqt2t0rpufjkvtzsrn5a8q5zffukek0rdtnq45vh4dgfslw4l46' },
  // Latest from: https://api.apollo.farm/api/graph?query=query+MyQuery+%7B%0A++vaults%28network%3A+osmo_test_5%29+%7B%0A++++label%0A++++contract_address%0A++%7D%0A%7D
  vaults: [aUSDC_OSMO_Config(ausdcOsmoVault), ATOM_OSMO_Config(atomOsmoVault)],
  swapperContractName: 'mars_swapper_osmosis',
  zapperContractName: 'mars_v2_zapper_osmosis',
  runTests: true,
  testActions: {
    allowedCoinsConfig: [
      { denom: uosmo, priceSource: { fixed: { price: '1' } }, grantCreditLine: true },
      {
        denom: aUSDC,
        priceSource: { geometric_twap: { pool_id: 5, window_size: 1800 } },
        grantCreditLine: true,
      },
      {
        denom: ausdcOsmoPool,
        priceSource: { xyk_liquidity_token: { pool_id: 6 } },
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
        baseToken: ausdcOsmoPool,
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
          pool_id: '1',
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
      denomOut: ausdcOsmoPool,
    },
  },
}
