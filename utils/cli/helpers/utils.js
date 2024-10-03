const { ethers } = require('ethers');
const path = require('path');
const readJSONFile = require('./readJSONFile');

const rootDir = path.resolve(__dirname, '../../../');

async function initializeChainContext(config = {}) {
  const provider = await getProvider(config.rpcUrl);
  const wallet = await getWallet(provider, config.rpcUrl);
  const chainId = await getChainId(provider);

  return { provider, wallet, chainId };
}

async function getStrategyFactoryContract(wallet, chainId) {
  const [, coreDeploymentData] = await getDeploymentData('core', chainId);
  const [, strategyFactoryABI] = await getABI('StrategyFactory');
  const strategyFactoryAddress = coreDeploymentData.addresses.strategyFactory;

  return new ethers.Contract(
    strategyFactoryAddress,
    strategyFactoryABI,
    wallet,
  );
}

async function getContractInstance(contractName, address, wallet) {
  const [, abi] = await getABI(contractName);
  return new ethers.Contract(address, abi, wallet);
}

async function getChainId(provider) {
  return provider.getNetwork().then((network) => network.chainId);
}

async function approveTokenIfNeeded(token, signer, spender, amount) {
  const signerAddress = await signer.getAddress();
  const currentAllowance = await token.allowance(signerAddress, spender);
  if (currentAllowance < amount) {
    const tx = await token.approve(spender, amount);
    await tx.wait();
    const newAllowance = await token.allowance(signerAddress, spender);
    console.log(
      `The Values were currentAllowance ${currentAllowance}, Amount ${amount}`,
    );
    console.log(
      `Allowance increased from ${currentAllowance} to ${newAllowance}`,
    );
  }
}

function parseTokenAmount(amount, decimals = 18) {
  return ethers.parseUnits(amount, decimals);
}

async function getTokenAddress(chainId) {
  const [, helloWorldDeploymentData] = await getDeploymentData(
    'hello-world',
    chainId,
  );
  return helloWorldDeploymentData.addresses.mockERC20;
}

async function getDeploymentData(deploymentType, chainId) {
  const filePath = path.resolve(
    rootDir,
    `contracts/deployments/${deploymentType}/${chainId}.json`,
  );
  const [error, data] = readJSONFile(filePath);
  if (error) {
    return [error, null];
  } else {
    return [null, data];
  }
}

async function getABI(contractName) {
  const filePath = path.resolve(rootDir, `abis/${contractName}.json`);
  const [error, abi] = readJSONFile(filePath);
  if (error) {
    return [error, null];
  } else {
    return [null, abi];
  }
}

async function getProvider(rpcUrl) {
  return new ethers.JsonRpcProvider(rpcUrl);
}

async function getWallet(provider, privateKey) {
  return new ethers.Wallet(privateKey, provider);
}

module.exports = {
  getDeploymentData,
  getABI,
  getProvider,
  getWallet,
  getStrategyFactoryContract,
  getTokenAddress,
  getContractInstance,
  getChainId,
  approveTokenIfNeeded,
  parseTokenAmount,
};
