const { ethers } = require('ethers');
const fs = require('fs');
const path = require('path');
require('dotenv').config();

async function createStrategyForToken(tokenAddress) {
  const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

  const chainId = 31337; // TODO: Make this dynamic
  const coreDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `../contracts/deployments/core/${chainId}.json`), 'utf8'));

  const strategyFactoryAddress = coreDeploymentData.addresses.strategyManager;
  const strategyFactoryABI = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../abis/StrategyFactory.json'), 'utf8'));

  const strategyFactory = new ethers.Contract(strategyFactoryAddress, strategyFactoryABI, wallet);

  try {
    const tx = await strategyFactory.deployNewStrategy(tokenAddress);
    await tx.wait();

    console.log("Strategy created for token");
    return true;
  } catch (error) {
    console.error("Error in creating strategy for token:", error);
    return [error.message, null];
  }
}

module.exports = createStrategyForToken;
