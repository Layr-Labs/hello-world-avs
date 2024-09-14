const { ethers } = require('ethers');
const fs = require('fs');
const path = require('path');
require('dotenv').config();

async function registerOperatorWithAVS() {
  const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);
  const signingKey = new ethers.Wallet(process.env.SIGNING_PRIVATE_KEY, provider);

  const chainId = 31337; // TODO: Make this dynamic
  const coreDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `../contracts/deployments/core/${chainId}.json`), 'utf8'));
  const helloWorldDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `../contracts/deployments/hello_world/${chainId}.json`), 'utf8'));

  const avsDirectoryAddress = coreDeploymentData.addresses.avsDirectory;
  const stakeRegistryAddress = helloWorldDeploymentData.addresses.stakeRegistry;
  const serviceManagerAddress = helloWorldDeploymentData.addresses.helloWorldServiceManager;

  const avsDirectoryABI = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../abis/AVSDirectory.json'), 'utf8'));
  const stakeRegistryABI = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../abis/ECDSAStakeRegistry.json'), 'utf8'));

  const avsDirectory = new ethers.Contract(avsDirectoryAddress, avsDirectoryABI, wallet);
  const stakeRegistry = new ethers.Contract(stakeRegistryAddress, stakeRegistryABI, wallet);

  try {
    const salt = ethers.randomBytes(32);
    const expiry = Math.floor(Date.now() / 1000) + 3600; // 1 hour from now

    const operatorAddress = await wallet.getAddress();
    const signingKeyAddress = await signingKey.getAddress();
    const operatorRegistrationDigestHash = await avsDirectory.calculateOperatorAVSRegistrationDigestHash(
      operatorAddress,
      serviceManagerAddress,
      salt,
      expiry
    );

    const signature = await wallet.signMessage(ethers.getBytes(operatorRegistrationDigestHash));

    const operatorSignature = {
      signature: signature,
      salt: salt,
      expiry: expiry
    };

    const tx = await stakeRegistry.registerOperatorWithSignature(operatorSignature, signingKeyAddress);
    await tx.wait();

    console.log("Operator registered to AVS successfully");
    console.log("Operator address:", operatorAddress);
    console.log("Signing key address:", signingKeyAddress);
    return true;
  } catch (error) {
    console.error("Error in registering operator with AVS:", error);
    return false;
  }
}

module.exports = registerOperatorWithAVS;

