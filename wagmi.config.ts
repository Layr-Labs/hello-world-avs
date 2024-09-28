import { foundry, actions } from '@wagmi/cli/plugins'
import fs from 'fs'
import path from 'path'

function parseDeployments(networkId: number = 31337) {
    const coreDeploymentPath = path.join(__dirname, 'contracts', 'deployments', 'core', `${networkId}.json`)
    const helloWorldDeploymentPath = path.join(__dirname, 'contracts', 'deployments', 'hello-world', `${networkId}.json`)

    const coreDeployment = JSON.parse(fs.readFileSync(coreDeploymentPath, 'utf8'))
    const helloWorldDeployment = JSON.parse(fs.readFileSync(helloWorldDeploymentPath, 'utf8'))

    const deployments: Record<string, Record<number, `0x${string}`>> = {}

    // Process core deployments
    for (const [contractName, address] of Object.entries(coreDeployment.addresses)) {
        if (!deployments[contractName]) {
            deployments[contractName] = {}
        }
        deployments[contractName][networkId] = address as `0x${string}`
    }

    // Process hello-world deployments
    for (const [contractName, address] of Object.entries(helloWorldDeployment.addresses)) {
        if (!deployments[contractName]) {
            deployments[contractName] = {}
        }
        deployments[contractName][networkId] = address as `0x${string}`
    }

    console.log(deployments)

    return deployments
}

const deployments = parseDeployments()

/** @type {import('@wagmi/cli').Config} */
export default {
    out: "./dist/generated.ts",
    plugins: [
        foundry({
            project: "./contracts",
            deployments: deployments
        }),
        actions()
    ],
};