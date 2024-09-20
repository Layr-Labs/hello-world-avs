const { Command } = require('commander');
const { ethers } = require('ethers');
const fs = require('fs');
const path = require('path');
require('dotenv').config();

async function createStrategyForToken(tokenAddress) {
  const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

  const chainId = 31337; // TODO: Make this dynamic
  const deploymentFilePath = path.resolve(__dirname, `../contracts/deployments/core/${chainId}.json`);
  const abiFilePath = path.resolve(__dirname, '../abis/StrategyFactory.json');

  let coreDeploymentData, strategyFactoryABI;

  try {
    if (!fs.existsSync(deploymentFilePath)) {
      throw new Error(`Deployment file not found: ${deploymentFilePath}`);
    }
    coreDeploymentData = JSON.parse(fs.readFileSync(deploymentFilePath, 'utf8'));

    if (!coreDeploymentData.addresses || !coreDeploymentData.addresses.strategyManager) {
      throw new Error("Invalid deployment data: strategyManager address not found");
    }

    if (!fs.existsSync(abiFilePath)) {
      throw new Error(`ABI file not found: ${abiFilePath}`);
    }
    strategyFactoryABI = JSON.parse(fs.readFileSync(abiFilePath, 'utf8'));
  } catch (error) {
    return [error.message, null];
  }

  const strategyFactoryAddress = coreDeploymentData.addresses.strategyManager;
  const strategyFactory = new ethers.Contract(strategyFactoryAddress, strategyFactoryABI, wallet);

  try {
    const tx = await strategyFactory.deployNewStrategy(tokenAddress);
    await tx.wait();
    return [null, tx];
  } catch (error) {
    return [error.message, null];
  }
}

const createStrategyCommand = new Command('create-strategy')
  .description('Create a strategy for a token')
  .argument('<tokenAddress>', 'The address of the token')
  .action(async (tokenAddress) => {
    const [error, result] = await createStrategyForToken(tokenAddress);
    if (error) {
      console.error('Failed to create strategy: \n', error);
    } else {
      console.log('Strategy created successfully');
    }
  });

module.exports = createStrategyCommand;
