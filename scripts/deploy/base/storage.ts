import { readFile, writeFile } from 'fs/promises'
import path from 'path'
import { StorageItems as StorageItems } from '../../types/storageItems'

export const ARTIFACTS_PATH = '../artifacts/'

export class Storage implements StorageItems {
  public addresses: StorageItems['addresses']
  public codeIds: StorageItems['codeIds']
  public actions: StorageItems['actions']
  public admin: StorageItems['admin']

  constructor(private chainId: string, items: StorageItems) {
    this.addresses = items.addresses
    this.codeIds = items.codeIds
    this.actions = items.actions
    this.admin = items.admin
  }

  static async load(chainId: string): Promise<Storage> {
    try {
      const data = await readFile(path.join(ARTIFACTS_PATH, `${chainId}.json`), 'utf8')
      const items = JSON.parse(data) as StorageItems
      return new this(chainId, items)
    } catch (e) {
      return new this(chainId, { addresses: {}, codeIds: {}, actions: {} })
    }
  }

  async save() {
    await writeFile(
      path.join(ARTIFACTS_PATH, `${this.chainId}.json`),
      JSON.stringify(this, null, 2),
    )
  }
}
