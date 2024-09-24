const { Command } = require('commander');
const { ethers } = require('ethers');
const path = require('path');
const readJSONFile = require('../helpers/readJSONFile');

require('dotenv').config();

async function createStrategyForToken() {
    const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);
    const privateKey = process.env.PRIVATE_KEY;
    const wallet = new ethers.Wallet(privateKey, provider);
    const chainId = await provider.getNetwork().then(network => network.chainId);

    const rootDir = path.resolve(__dirname, '../../../');
    const coreDeploymentFilePath = path.resolve(rootDir, `contracts/deployments/core/${chainId}.json`);
    const helloWorldDeploymentFilePath = path.resolve(rootDir, `contracts/deployments/hello-world/${chainId}.json`);
    const strategyFactoryABIPath = path.resolve(rootDir, 'abis/StrategyFactory.json');

    const [, coreDeploymentData] = readJSONFile(coreDeploymentFilePath);
    const [, helloWorldDeploymentData] = readJSONFile(helloWorldDeploymentFilePath);
    const [, strategyFactoryABI] = readJSONFile(strategyFactoryABIPath);
    const strategyFactoryAddress = coreDeploymentData.addresses.strategyFactory;
    const mockERC20Address = helloWorldDeploymentData.addresses.mockERC20;
    const strategyFactoryContract = new ethers.Contract(
        strategyFactoryAddress,
        strategyFactoryABI,
        wallet
    );

    try {
        const strategyAddress = await strategyFactoryContract.deployedStrategies(mockERC20Address);
        if (strategyAddress !== ethers.ZeroAddress) {
            return [null, strategyAddress];
        }
        const tx = await strategyFactoryContract.deployNewStrategy(mockERC20Address);
        await tx.wait();
        const newStrategyAddress = await strategyFactoryContract.deployedStrategies(mockERC20Address);

        return [null, newStrategyAddress];
    } catch (error) {
        return [error.message, null];
    }
}

const createStrategyCommand = new Command('create-strategy')
    .description('Create a strategy for a token')
    .action(async () => {
        const [error, result] = await createStrategyForToken();
        if (error) {
            console.error('Failed to create strategy: \n', error);
        } else {
            console.log('Strategy operation completed successfully');
            console.log('Strategy address:', result);
        }
    });

module.exports = { createStrategyCommand, createStrategyForToken };
