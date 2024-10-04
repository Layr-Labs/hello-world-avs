import { createConfig, http } from '@wagmi/core'
import { anvil } from '@wagmi/core/chains'
import { createClient, publicActions, walletActions } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import * as dotenv from "dotenv"
import path from 'path'
import fs from 'fs'

dotenv.config()

// Check if the process.env object is empty
if (!Object.keys(process.env).length) {
    throw new Error("process.env object is empty")
}

const account = privateKeyToAccount(process.env.PRIVATE_KEY as `0x${string}`)

const config = createConfig({
    chains: [anvil],
    client({ chain }) {
        return createClient({ 
            account: account,
            chain, 
            transport: http() 
        }).extend(publicActions).extend(walletActions)
    },
})

const chainId = 31337

const __dirname = process.cwd()

// Load core deployment data
const avsDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `./contracts/deployments/hello-world/${chainId}.json`), 'utf8'))
const coreDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `./contracts/deployments/core/${chainId}.json`), 'utf8'))

const delegationManagerAddress = coreDeploymentData.addresses.delegation
const avsDirectoryAddress = coreDeploymentData.addresses.avsDirectory
const helloWorldServiceManagerAddress = avsDeploymentData.addresses.helloWorldServiceManager
const stakeRegistryAddress = avsDeploymentData.addresses.stakeRegistry

export {
    account,
    config,
    chainId,
    delegationManagerAddress,
    avsDirectoryAddress,
    helloWorldServiceManagerAddress,
    stakeRegistryAddress
}


