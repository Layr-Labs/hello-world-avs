const { Command } = require('commander');
const { ethers } = require('ethers');
const {
  getProvider,
  getWallet,
  getTokenAddress,
  getABI,
  getChainId,
} = require('../helpers/utils');

require('dotenv').config();

async function mintMockTokens(config = {}) {
  const provider = await getProvider(config.rpcUrl);
  const wallet = await getWallet(provider, config.privateKey);
  const chainId = await getChainId(provider);

  const tokenAddress = config.tokenAddress || (await getTokenAddress(chainId));
  const [, tokenABI] = await getABI('ERC20Mock');

  const token = new ethers.Contract(tokenAddress, tokenABI, wallet);

  try {
    const amount = ethers.parseUnits(config.amount, 18); // Assuming 18 decimals, adjust if needed
    const tx = await token.mint(wallet.address, amount);
    return [null, tx];
  } catch (error) {
    return [error, null];
  }
}

const mintMockTokensCommand = new Command('mint-mock-tokens')
  .description('Mint mock tokens to the caller')
  .option('-t, --token-address [address]', 'Token address to mint')
  .option('-a, --amount [amount]', 'Amount to mint')
  .option('-r, --rpc-url [url]', 'Custom RPC URL')
  .option('-p, --private-key [key]', 'Custom private key')
  .action(async (options) => {
    const config = {
      tokenAddress: options.tokenAddress,
      amount: options.amount,
      rpcUrl: options.rpcUrl || process.env.RPC_URL,
      privateKey: options.privateKey || process.env.PRIVATE_KEY,
    };

    const [error, result] = await mintMockTokens(config);

    if (error) {
      console.error('Failed to mint mock tokens: \n', error);
    } else {
      console.log('Mock tokens minted successfully');
      console.log('Transaction hash:', result.transactionHash);
    }
  });

module.exports = { mintMockTokensCommand, mintMockTokens };
