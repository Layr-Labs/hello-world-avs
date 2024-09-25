const { Command } = require('commander')
const { ethers } = require('ethers')
const {
  getProvider,
  getWallet,
  getDeploymentData,
  getABI,
} = require('../helpers/utils')

require('dotenv').config()

async function registerAsOperator(config = {}) {
  const provider = await getProvider(config.rpcUrl)
  const wallet = await getWallet(provider, config.privateKey)
  const chainId = await provider.getNetwork().then((network) => network.chainId)

  const coreDeploymentData = await getDeploymentData('core', chainId)
  const delegationManagerABI = await getABI('DelegationManager')
  const delegationManagerAddress = coreDeploymentData.addresses.delegation

  const delegationManager = new ethers.Contract(
    delegationManagerAddress,
    delegationManagerABI,
    wallet
  )

  try {
    // Check if the address is already registered as an operator
    const isOperator = await delegationManager.isOperator(wallet.address)
    if (isOperator) {
      console.log('Address is already registered as an operator')
      return [null, { alreadyRegistered: true }]
    }

    /// TODO: hack make better
    const delegatedAddress = await delegationManager.delegatedTo(wallet.address)
    if (delegatedAddress === wallet.address) {
      return [null, true]
    }
    const operatorDetails = {
      __deprecated_earningsReceiver: ethers.ZeroAddress,
      delegationApprover:
        config.operatorDetails.delegationApprover || ethers.ZeroAddress,
      stakerOptOutWindowBlocks:
        config.operatorDetails.stakerOptOutWindowBlocks || 0,
    }
    const tx = await delegationManager.registerAsOperator(operatorDetails, '')
    return [null, tx]
  } catch (error) {
    return [error, null]
  }
}

const registerAsOperatorCommand = new Command('register-as-operator')
  .description('Register as an operator')
  .option('-d, --delegation-approver [address]', 'Delegation approver address')
  .option(
    '-w, --staker-opt-out-window [blocks]',
    'Staker opt-out window in blocks'
  )
  .option('-r, --rpc-url [url]', 'Custom RPC URL')
  .option('-p, --private-key [key]', 'Custom private key')
  .action(async (options) => {
    const config = {
      operatorDetails: {
        delegationApprover: options.delegationApprover || ethers.ZeroAddress,
        stakerOptOutWindowBlocks: options.stakerOptOutWindow
          ? parseInt(options.stakerOptOutWindow)
          : 0,
      },
      rpcUrl: options.rpcUrl || process.env.RPC_URL,
      privateKey: options.privateKey || process.env.PRIVATE_KEY,
    }

    const [error, result] = await registerAsOperator(config)

    if (error) {
      console.error('Failed to register as operator: \n', error)
    } else {
      console.log('Successfully registered as operator')
      console.log('Transaction hash:', result.hash)
    }
  })

module.exports = { registerAsOperatorCommand, registerAsOperator }
