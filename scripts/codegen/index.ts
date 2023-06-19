import codegen from '@cosmwasm/ts-codegen'
import { join, resolve } from 'path'
import { printGreen, printRed } from '../utils/chalk'
import { readdir, rename, rm } from 'fs/promises'
import simpleGit from 'simple-git'

const generateTypes = async () => {
  const schemasDir = resolve(join(__dirname, '../../../schemas'))
  const schemas = await readdir(schemasDir)

  for (const schema of schemas) {
    try {
      await codegen({
        contracts: [`${schemasDir}/${schema}`],
        outPath: `./types/generated/${schema}`,
        options: {
          types: {
            enabled: true,
          },
          client: {
            enabled: true,
          },
          reactQuery: {
            enabled: true,
            optionalClient: true,
            version: 'v4',
            mutations: true,
            queryKeys: true,
          },
          messageComposer: {
            enabled: true,
          },
        },
      })
      printGreen(`Success ✨ ${schema} types generated`)
    } catch (e) {
      printRed(`Error with ${schema}: ${e}`)
    }
  }
}

const fetchSchemafromGithub = async ({
  githubRepo,
  pathToSchema,
  commit,
}: {
  githubRepo: string
  pathToSchema: string
  commit: string
}) => {
  const git = simpleGit()
  await git.clone(githubRepo)
  const repoDirName = githubRepo.split('/').pop()!
  await git.cwd({ path: `./${repoDirName}`, root: true })
  await git.checkout(commit)
  const schemaDirName = pathToSchema.split('/').pop()!
  await rename(pathToSchema, `../schemas/${schemaDirName}`)
  await rm(`./${repoDirName}`, { recursive: true, force: true })
}

void (async function () {
  await fetchSchemafromGithub({
    // TODO: HLS PR should update the commit hash to latest
    githubRepo: 'https://github.com/mars-protocol/mars-common',
    commit: 'f1077562d3471e01f4e78a14ab30b019d578b3c1',
    pathToSchema: './mars-common/schemas/mars-params',
  })
  await generateTypes()
})()
