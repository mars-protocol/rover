// @ts-nocheck

import { getWallet, setupDeployer } from './setupDeployer'
import { printGray, printRed, printYellow } from '../../utils/chalk'
import { DeploymentConfig } from '../../types/config'
import { wasmFile } from '../../utils/environment'
import { StargateClient } from '@cosmjs/stargate'

export interface TaskRunnerProps {
  config: DeploymentConfig
  swapperContractName: string
  zapperContractName: string
}

export const taskRunner = async ({
  config,
  swapperContractName,
  zapperContractName,
}: TaskRunnerProps) => {
  const deployer = await setupDeployer(config)
  try {
    // Upload contracts
    // await deployer.upload('accountNft', wasmFile('mars_account_nft'))
    // await deployer.upload('mockVault', wasmFile('mars_mock_vault'))
    // await deployer.upload('marsOracleAdapter', wasmFile('mars_oracle_adapter'))
    // await deployer.upload('swapper', wasmFile(swapperContractName))
    // await deployer.upload('zapper', wasmFile(zapperContractName))
    // await deployer.upload('creditManager', wasmFile('mars_credit_manager'))

    const swapper = 'osmo1940km39gjdxxprg5q0220d2upz9ff3eju5jmcrfpp3dcehct4a9spm8yxl'
    const uatom = 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2'
    const execRes = await deployer.cwClient.execute(
      deployer.deployerAddr,
      swapper,
      {
        set_route: {
          denom_in: uatom,
          denom_out: 'uosmo',
          route: [
            {
              pool_id: '1',
              token_out_denom: 'uosmo',
            },
          ],
        },
      },
      'auto',
    )
    printGray(JSON.stringify(execRes))

    const queryRes = await deployer.cwClient.queryContractSmart(swapper, {
      routes: { limit: undefined, start_after: undefined },
    })
    printGray(JSON.stringify(queryRes))

    // // Set contracts owner
    // deployer.setOwnerAddr()
    //
    // // Instantiate contracts
    // await deployer.instantiateMockVault()
    // await deployer.instantiateMarsOracleAdapter()
    // await deployer.instantiateSwapper()
    // await deployer.instantiateZapper()
    // await deployer.instantiateCreditManager()
    // await deployer.instantiateNftContract()
    // await deployer.transferNftContractOwnership()
    // await deployer.saveDeploymentAddrsToFile()
    //
    // // Test basic user flows
    // if (config.testActions) {
    //   await deployer.grantCreditLines()
    //   await deployer.setupOraclePrices()
    //   await deployer.setupRedBankMarketsForZapDenoms()
    //
    //   const rover = await deployer.newUserRoverClient(config.testActions)
    //   await rover.createCreditAccount()
    //   await rover.deposit()
    //   await rover.borrow()
    //   await rover.swap()
    //   await rover.repay()
    //   await rover.withdraw()
    //
    //   const vaultConfig = config.vaults[0]
    //   const info = await rover.getVaultInfo(vaultConfig)
    //   await rover.zap(info.tokens.base_token)
    //   await rover.vaultDeposit(vaultConfig, info)
    //   if (info.lockup) {
    //     await rover.vaultRequestUnlock(vaultConfig, info)
    //   } else {
    //     await rover.vaultWithdraw(vaultConfig, info)
    //     await rover.unzap(info.tokens.base_token)
    //   }
    //   await rover.refundAllBalances()
    // }

    printYellow('COMPLETE')
  } catch (e) {
    printRed(e)
  } finally {
    // await deployer.saveStorage()
  }
}
