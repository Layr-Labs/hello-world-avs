const { ethers } = require('ethers');
const path = require('path');
const readJSONFile = require('./readJSONFile');

const rootDir = path.resolve(__dirname, '../../../');

async function getStrategyFactoryContract(wallet, chainId) {
	const coreDeploymentData = await getDeploymentData('core', chainId);
	const strategyFactoryABI = await getABI('StrategyFactory');
	const strategyFactoryAddress = coreDeploymentData.addresses.strategyFactory;

	return new ethers.Contract(
		strategyFactoryAddress,
		strategyFactoryABI,
		wallet
	);
}

async function getTokenAddress(chainId) {
	const helloWorldDeploymentData = await getDeploymentData('hello-world', chainId);
	return helloWorldDeploymentData.addresses.mockERC20;
}

async function getDeploymentData(deploymentType, chainId) {
  const filePath = path.resolve(rootDir, `contracts/deployments/${deploymentType}/${chainId}.json`);
  const [error, data] = readJSONFile(filePath);
  if (error) {
    throw new Error(`Failed to read deployment data: ${error}`);
  }
  return data;
}

async function getABI(contractName) {
  const filePath = path.resolve(rootDir, `abis/${contractName}.json`);
  const [error, abi] = readJSONFile(filePath);
  if (error) {
    throw new Error(`Failed to read ABI: ${error}`);
  }
  return abi;

}

async function getProvider(rpcUrl) {
  return new ethers.JsonRpcProvider(rpcUrl);
}

async function getWallet(provider, privateKey) {
  return new ethers.Wallet(privateKey, provider);
}

module.exports = { getDeploymentData, getABI, getProvider, getWallet, getStrategyFactoryContract, getTokenAddress };