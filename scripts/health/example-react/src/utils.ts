import {Positions} from '../../../types/generated/mars-credit-manager/MarsCreditManager.types'

import init, {compute_health_js, max_borrow_estimate_js, max_withdraw_estimate_js,} from '../../pkg-web'
import {HealthValuesResponse} from '../../../types/generated/mars-rover-health-types/MarsRoverHealthTypes.types'
import {DataFetcher} from '../../DataFetcher'

const getFetcher = (cmAddress: string) => {
    return new DataFetcher(
        compute_health_js,
        max_withdraw_estimate_js,
        max_borrow_estimate_js,
        'osmo1m83kw2vehyt9urxf79qa9rxk8chgs4464e5h8s37yhnw3pwauuqq7lux8r',
        'osmo156elt2tp5455q9a6vfrvnpncxyd33cxm9z2lgguwg6dgws9tedps5tq3rc',
        'osmo1vxpdcw092n9rngvekve8g324c2yjx56496ck98ag4sdxr4p4zd4q0wr7x6',
        'https://rpc.devnet.osmosis.zone/',
    )
}

export const fetchPositions = async (cmAddress: string, accountId: string): Promise<Positions> => {
    const dataFetcher = getFetcher(cmAddress)
    return await dataFetcher.fetchPositions(accountId)
}

export const fetchHealth = async (
    cmAddress: string,
    accountId: string,
): Promise<HealthValuesResponse> => {
    await init()
    const dataFetcher = getFetcher(cmAddress)
    return await dataFetcher.computeHealth(accountId)
}
