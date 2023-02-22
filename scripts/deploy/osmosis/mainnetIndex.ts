import { taskRunner } from '../base'
import { osmosisMainnetConfig } from './config'

void (async function () {
  await taskRunner({
    config: osmosisMainnetConfig,
    swapperContractName: 'mars_swapper_osmosis',
    zapperContractName: 'mars_zapper_osmosis',
  })
})()
