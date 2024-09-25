const { Command } = require('commander');
const { ethers } = require('ethers');
const {
  getProvider,
  getWallet,
  getStrategyFactoryContract,
  getTokenAddress,
  getDeploymentData,
  getABI,
  getChainId,
  approveTokenIfNeeded,
} = require('../helpers/utils');

require('dotenv').config();

async function depositIntoStrategy(config = {}) {
  const provider = await getProvider(config.rpcUrl);
  const wallet = await getWallet(provider, config.privateKey);
  const chainId = await getChainId(provider);

  const [, coreDeploymentData] = await getDeploymentData('core', chainId);
  const [, strategyManagerABI] = await getABI('StrategyManager');
  const strategyManagerAddress = coreDeploymentData.addresses.strategyManager;

  const strategyManager = new ethers.Contract(
    strategyManagerAddress,
    strategyManagerABI,
    wallet,
  );

  const tokenAddress = config.tokenAddress || (await getTokenAddress(chainId));
  const token = new ethers.Contract(
    tokenAddress,
    await getABI('ERC20Mock'),
    wallet,
  );

  try {
    const strategyFactory = await getStrategyFactoryContract(wallet, chainId);

    const amount = ethers.parseUnits(config.amount, 18); // Assuming 18 decimals, adjust if needed
    const strategy = await strategyFactory.deployedStrategies(tokenAddress);

    if (strategy === ethers.ZeroAddress) {
      return [new Error('Strategy not deployed for this token'), null];
    }

    /// TODO: FIX THIS
    // await approveTokenIfNeeded(token, wallet, strategyManagerAddress, amount);

    const tx = await strategyManager.depositIntoStrategy(
      strategy,
      tokenAddress,
      amount,
    );
    return [null, tx];
  } catch (error) {
    console.error(error);
    return [error, null];
  }
}

const depositIntoStrategyCommand = new Command('deposit-into-strategy')
  .description('Deposit tokens into a strategy')
  .option('-t, --token-address [address]', 'Token address to deposit')
  .option('-a, --amount [amount]', 'Amount to deposit')
  .option('-r, --rpc-url [url]', 'Custom RPC URL')
  .option('-p, --private-key [key]', 'Custom private key')
  .action(async (options) => {
    const config = {
      tokenAddress: options.tokenAddress,
      amount: options.amount,
      rpcUrl: options.rpcUrl || process.env.RPC_URL,
      privateKey: options.privateKey || process.env.PRIVATE_KEY,
    };

    const [error, result] = await depositIntoStrategy(config);

    if (error) {
      console.error('Failed to deposit into strategy: \n', error);
    } else {
      console.log('Deposit into strategy completed successfully');
      console.log('Transaction hash:', result.transactionHash);
    }
  });

module.exports = { depositIntoStrategyCommand, depositIntoStrategy };
