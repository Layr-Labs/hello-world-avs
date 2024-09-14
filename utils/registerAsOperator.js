const { ethers } = require('ethers');
const fs = require('fs');
const path = require('path');
require('dotenv').config();

async function registerAsOperator() {
  const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider);

  const chainId = 31337; // TODO: Make this dynamic
  const coreDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `../contracts/deployments/core/${chainId}.json`), 'utf8'));

  const delegationManagerAddress = coreDeploymentData.addresses.delegation;
  const delegationManagerABI = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../abis/IDelegationManager.json'), 'utf8'));

  const delegationManager = new ethers.Contract(delegationManagerAddress, delegationManagerABI, wallet);

  try {
    const tx = await delegationManager.registerAsOperator({
      __deprecated_earningsReceiver: await wallet.getAddress(),
      delegationApprover: "0x0000000000000000000000000000000000000000",
      stakerOptOutWindowBlocks: 0
    }, "");
    
    await tx.wait();
    console.log("Operator registered to Core EigenLayer contracts");
    return true;
  } catch (error) {
    console.error("Error in registering as operator:", error);
    return false;
  }
}

module.exports = registerAsOperator;
