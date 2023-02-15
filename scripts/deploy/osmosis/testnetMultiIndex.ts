import { taskRunner } from '../base'
import { osmosisTestnetMultisigConfig } from './config'

void (async function () {
  await taskRunner({
    config: osmosisTestnetMultisigConfig,
    swapperContractName: 'mars_swapper_osmosis',
    zapperContractName: 'mars_zapper_osmosis',
  })
})()
