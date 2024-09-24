const { ethers } = require('ethers')
const fs = require('fs')
const path = require('path')
require('dotenv').config()

async function depositTokenIntoStrategy(tokenAddress, amount) {
  const provider = new ethers.JsonRpcProvider(process.env.RPC_URL)
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider)

  const chainId = 31337 // TODO: Make this dynamic
  const coreDeploymentData = JSON.parse(
    fs.readFileSync(
      path.resolve(__dirname, `../contracts/deployments/core/${chainId}.json`),
      'utf8'
    )
  )

  const strategyManagerAddress = coreDeploymentData.addresses.strategyManager
  const strategyManagerABI = JSON.parse(
    fs.readFileSync(
      path.resolve(__dirname, '../abis/IStrategyManager.json'),
      'utf8'
    )
  )

  const strategyFactoryAddress = coreDeploymentData.addresses.strategyFactory
  const strategyFactoryABI = JSON.parse(
    fs.readFileSync(
      path.resolve(__dirname, '../abis/IStrategyFactory.json'),
      'utf8'
    )
  )
  const tokenABI = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, '../abis/ERC20Mock.json'), 'utf8')
  )

  const strategyManager = new ethers.Contract(
    strategyManagerAddress,
    strategyManagerABI,
    wallet
  )
  const strategyFactory = new ethers.Contract(
    strategyFactoryAddress,
    strategyFactoryABI,
    wallet
  )
  const token = new ethers.Contract(tokenAddress, tokenABI, wallet)

  try {
    let strategy = await strategyFactory.deployedStrategies(tokenAddress)
    if (strategy === ethers.ZeroAddress) {
      console.log('Create strategy for the token')
      throw error()
    }

    console.log('Approving token transfer...')
    const approveTx = await token.approve(strategyManagerAddress, amount)
    await approveTx.wait()
    console.log('Token transfer approved')

    console.log('Depositing into strategy...')
    const depositTx = await strategyManager.depositIntoStrategy(
      strategy,
      tokenAddress,
      amount
    )
    await depositTx.wait()
    console.log('Deposit successful')
    return true
  } catch (error) {
    console.error('Error in depositing token into strategy:', error)
    return false
  }
}

module.exports = depositTokenIntoStrategy
