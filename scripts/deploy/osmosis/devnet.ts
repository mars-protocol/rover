import { taskRunner } from '../base'
import { DeploymentConfig } from '../../types/config'

const uosmo = 'uosmo'
const uatom = 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2'
const axlUSDC = 'ibc/D189335C6E4A68B513C10AB227BF1C1D38C746766278BA3EEB4FB14124F1D858'
const gammPool1 = 'gamm/pool/1'
const gammPool678 = 'gamm/pool/678'

export const osmosisDevnetConfig: DeploymentConfig = {
  // multisigAddr: 'osmo14w4x949nwcrqgfe53pxs3k7x53p0gvlrq34l5n',
  allowedCoins: [uosmo, uatom, axlUSDC, gammPool1, gammPool678],
  chain: {
    baseDenom: uosmo,
    defaultGasPrice: 0.1,
    id: 'devnet',
    prefix: 'osmo',
    rpcEndpoint: 'https://rpc.devnet.osmosis.zone',
  },
  deployerMnemonic: 'TODO',
  maxUnlockingPositions: '1',
  maxValueForBurn: '10000',
  // oracle and redbank contract addresses can be found:  https://github.com/mars-protocol/red-bank/blob/master/README.md#osmosis-1
  addressProvider: { addr: 'osmo1m74wv3xew5dsy2thf3jp0xadg8pdrk4h8ym70z0ehfwxl8a547asxaaphj' },
  oracle: { addr: 'osmo1lqgdq9u8zhcvwwwz3xjswactrtq6qzptmlzlh6xspl34dxq32uhqp87uaf' },
  redBank: { addr: 'osmo1ul4msjc3mmaxsscdgdtjds85rg50qrepvrczp0ldgma5mm9xv8yqh59fvl' },
  incentives: { addr: 'osmo15v8jqq6aqhsuykdgdevx3qqcj9lp4h27ypsycds4cmv6er9qv0vs3h5r5a' },
  params: { addr: 'osmo1maqs3qvslrjaq8xz9402shucnr4wzdujty8lr7ux5z5rnj989lwsla84q0' },
  swapper: { addr: 'osmo1sgffmlhsa49xu7nk8nakmhzm8dtngslh05x3238z4jm79s5346ssujz8gt' },
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
