import { taskRunner } from '../base'
import { DeploymentConfig } from '../../types/config'

const osmo = 'uosmo'
const atom = 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2'
// const axl = 'ibc/903A61A498756EA560B85A85132D3AEE21B5DEDD41213725D22ABF276EA6945E'
// const stAtom = 'ibc/C140AFD542AE77BD7DCC83F13FDD8C5E5BB8C4929785E6EC2F4C636F98F17901'
const wbtc = 'ibc/D1542AA8762DB13087D8364F3EA6509FD6F009A34F00426AF9E4F9FA85CBBF1F'
const axlUSDC = 'ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858'
const eth = 'ibc/EA1D43981D5C9A1C4AAEA9C23BB1D4FA126BA9BC7020A25E0AE4AA841EA25DC5'

const defaultCreditLine = '1000000000000'

export const osmosisDevnetConfig: DeploymentConfig = {
  // multisigAddr: 'osmo14w4x949nwcrqgfe53pxs3k7x53p0gvlrq34l5n',
  creditLineCoins: [ // AXL and stAtom has borrowing disabled
    { denom: osmo, creditLine: defaultCreditLine },
    { denom: atom, creditLine: defaultCreditLine },
    { denom: wbtc, creditLine: defaultCreditLine },
    { denom: axlUSDC, creditLine: defaultCreditLine },
    { denom: eth, creditLine: '1000000000000000000000' },
  ],
  chain: {
    baseDenom: osmo,
    defaultGasPrice: 0.1,
    id: 'devnet',
    prefix: 'osmo',
    rpcEndpoint: 'https://rpc.devnet.osmosis.zone',
  },
  deployerMnemonic: 'TODO',
  maxUnlockingPositions: '1',
  maxValueForBurn: '10000',
  // oracle and redbank contract addresses can be found:  https://github.com/mars-protocol/red-bank/blob/master/README.md#osmosis-1
  addressProvider: { addr: 'osmo1mgt372t6vjmxpax53746ywcecum7wg0ye5ptaf42rjwat9qcqnhqgvfp8q' },
  oracle: { addr: 'osmo1ht8js7p6y5jxthze8hy3egfxflh8t9mvkl79w75mg6atu4ssfc0s7z8jd6' },
  redBank: { addr: 'osmo1wpqx4mhe5hmgte8s4etam4syfxjt83zwvejhgsmcludfpt5hd6kqg5zwy5' },
  incentives: { addr: 'osmo1kg8uzrhqnjp0a4rp6c6wtnl3y4prth7v54h5gwnkwgtn82yealxsdysvp0' },
  params: { addr: 'osmo1ytn4v3pd9ecklqf32pusephxpnfvnev7657rprr3jh43rchygvmq3fjp85' },
  swapper: { addr: 'osmo1x6ttpp3vpzaknxcc25u0mjrlnagu3agvwnyl0wpwhtjdljaw8aks6e28hv' },
  runTests: false,
  vaults: [],
  zapperContractName: 'mars_v2_zapper_osmosis',
}

void (async function () {
  await taskRunner({
    config: osmosisDevnetConfig,
    label: 'devnet',
  })
})()
