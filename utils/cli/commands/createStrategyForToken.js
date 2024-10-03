const { Command } = require('commander');
const { ethers } = require('ethers');
const {
  getProvider,
  getWallet,
  getStrategyFactoryContract,
  getTokenAddress,
  getChainId,
} = require('../helpers/utils');

require('dotenv').config();

async function deployOrGetStrategy(strategyFactory, tokenAddress) {
  try {
    const strategy = await strategyFactory.deployedStrategies(tokenAddress);
    if (strategy !== ethers.ZeroAddress) {
      return [null, strategy];
    }
  } catch (error) {
    return [error.message, null];
  }

  try {
    await strategyFactory.deployNewStrategy(tokenAddress);
    const strategy = await strategyFactory.deployedStrategies(tokenAddress);
    return [null, strategy];
  } catch (error) {
    return [error.message, null];
  }
}

async function createStrategyForToken(config = {}) {
  const provider = await getProvider(config.rpcUrl);
  const wallet = await getWallet(provider, config.privateKey);
  const chainId = await getChainId(provider);

  const strategyFactoryContract = await getStrategyFactoryContract(
    wallet,
    chainId,
  );
  const tokenAddress = config.tokenAddress || (await getTokenAddress(chainId));

  return deployOrGetStrategy(strategyFactoryContract, tokenAddress);
}

const createStrategyCommand = new Command('create-strategy')
  .description('Create a strategy for a token')
  .option(
    '-t, --token-address [address]',
    'Token address to create strategy for',
  )
  .option('-r, --rpc-url [url]', 'Custom RPC URL')
  .option('-p, --private-key [key]', 'Custom private key')
  .action(async (options) => {
    const config = {
      tokenAddress: options.tokenAddress,
      rpcUrl: options.rpcUrl || process.env.RPC_URL,
      privateKey: options.privateKey || process.env.PRIVATE_KEY,
    };

    const [error, result] = await createStrategyForToken(config);

    if (error) {
      console.error('Failed to create strategy: \n', error);
    } else {
      console.log('Strategy operation completed successfully');
      console.log('Strategy address:', result);
    }
  });

module.exports = { createStrategyCommand, createStrategyForToken };
