import { setupDeployer } from './setupDeployer'
import { printRed, printYellow } from '../../utils/chalk'
import { DeploymentConfig } from '../../types/config'
import { wasmFile } from '../../utils/environment'
import {
  MarsCreditManagerClient,
  MarsCreditManagerQueryClient,
} from '../../types/generated/mars-credit-manager/MarsCreditManager.client'
import { MarsMockVaultQueryClient } from '../../types/generated/mars-mock-vault/MarsMockVault.client'
import { MarsOracleAdapterQueryClient } from '../../types/generated/mars-oracle-adapter/MarsOracleAdapter.client'

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
    await deployer.upload('accountNft', wasmFile('mars_account_nft'))
    await deployer.upload('mockVault', wasmFile('mars_mock_vault'))
    await deployer.upload('marsOracleAdapter', wasmFile('mars_oracle_adapter'))
    await deployer.upload('swapper', wasmFile(swapperContractName))
    await deployer.upload('zapper', wasmFile(zapperContractName))
    await deployer.upload('creditManager', wasmFile('mars_credit_manager'))

    // Instantiate contracts
    await deployer.instantiateMockVault()
    await deployer.instantiateMarsOracleAdapter()
    await deployer.instantiateSwapper()
    await deployer.instantiateZapper()
    await deployer.instantiateCreditManager()
    await deployer.instantiateNftContract()
    await deployer.transferNftContractOwnership()
    await deployer.saveDeploymentAddrsToFile()

    // @ts-ignore
    const cmExec = new MarsCreditManagerClient(
      deployer.cwClient,
      deployer.deployerAddr,
      deployer.storage.addresses.creditManager!,
    )
    const cmQuery = new MarsCreditManagerQueryClient(
      deployer.cwClient,
      deployer.storage.addresses.creditManager!,
    )

    // const result = await cmExec.createCreditAccount('auto')
    // console.log(JSON.stringify(result, null, 2))
    const accountId = '9'
    // await cmExec.updateCreditAccount(
    //   { accountId, actions: [{ deposit: { denom: 'uosmo', amount: '10000000' } }] },
    //   'auto',
    //   undefined,
    //   [{ denom: 'uosmo', amount: '10000000' }],
    // )
    const positions = await cmQuery.positions({ accountId })
    console.log('current positions', JSON.stringify(positions, null, 2))

    const vaultInfos = await cmQuery.vaultsInfo({})
    console.log('vaultInfos', JSON.stringify(vaultInfos, null, 2))

    const vQuery = new MarsMockVaultQueryClient(
      deployer.cwClient,
      'osmo1v40lnedgvake8p7f49gvqu0q3vc9sx3qpc0jqtyfdyw25d4vg8us38an37',
    )
    const pQuery = new MarsOracleAdapterQueryClient(
      deployer.cwClient,
      deployer.storage.addresses.marsOracleAdapter!,
    )
    const apolloVault = await vQuery.info()
    console.log('mock vault info', JSON.stringify(apolloVault, null, 2))

    // const allPricingInfos = await pQuery.allPricingInfo({})
    // console.log('allPricingInfos', JSON.stringify(allPricingInfos, null, 2))
    //
    // const lpPrice = await pQuery.price({ denom: apolloVault.base_token })
    // console.log('lpPrice', JSON.stringify(lpPrice, null, 2))
    const lpTokenPrice = await pQuery.price({ denom: apolloVault.base_token })
    console.log('lpTokenPrice', JSON.stringify(lpTokenPrice, null, 2))

    const vaultTokenPriceQuery = await pQuery.price({ denom: apolloVault.vault_token })
    console.log('vaultTokenPriceQuery', JSON.stringify(vaultTokenPriceQuery, null, 2))

    const totalVaultTokenSupply = await vQuery.totalVaultTokenSupply()
    console.log('totalVaultTokenSupply', totalVaultTokenSupply)

    const totalUnderlying = await vQuery.previewRedeem({ amount: totalVaultTokenSupply })
    console.log('totalUnderlying', totalUnderlying)

    const underlyingPerVaultCoin = parseInt(totalUnderlying) / parseInt(totalVaultTokenSupply)
    console.log('underlyingPerVaultCoin', underlyingPerVaultCoin)

    const vaultTokenPrice = parseFloat(lpTokenPrice.price) * underlyingPerVaultCoin
    console.log('vaultTokenPrice', vaultTokenPrice)

    // await cmExec.updateCreditAccount({
    //   accountId,
    //   actions: [
    //     {
    //       borrow: {
    //         denom: 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2',
    //         amount: '1522142',
    //       },
    //     },
    //     {
    //       swap_exact_in: {
    //         coin_in: {
    //           denom: 'uosmo',
    //           amount: {
    //             exact: '2450002',
    //           },
    //         },
    //         denom_out: 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2',
    //         slippage: '0.02',
    //       },
    //     },
    //     {
    //       provide_liquidity: {
    //         coins_in: [
    //           {
    //             denom: 'ibc/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2',
    //             amount: 'account_balance',
    //           },
    //           {
    //             denom: 'uosmo',
    //             amount: 'account_balance',
    //           },
    //         ],
    //         lp_token_out: 'gamm/pool/1',
    //         minimum_receive: '86251146791125570000',
    //       },
    //     },
    //   ],
    // })

    await cmExec.updateCreditAccount({
      accountId,
      actions: [
        {
          enter_vault: {
            coin: {
              denom: 'gamm/pool/1',
              amount: 'account_balance',
            },
            vault: {
              address: 'osmo1v40lnedgvake8p7f49gvqu0q3vc9sx3qpc0jqtyfdyw25d4vg8us38an37',
            },
          },
        },
      ],
    })

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
    await deployer.saveStorage()
  }
}
