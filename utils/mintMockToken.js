const { ethers } = require('ethers')
const fs = require('fs')
const path = require('path')
require('dotenv').config()

async function mintMockToken() {
  const provider = new ethers.JsonRpcProvider(process.env.RPC_URL)
  const wallet = new ethers.Wallet(process.env.PRIVATE_KEY, provider)

  const chainId = 31337 // TODO: Make this dynamic
  const helloWorldDeploymentData = JSON.parse(
    fs.readFileSync(
      path.resolve(
        __dirname,
        `../contracts/deployments/hello-world/${chainId}.json`
      ),
      'utf8'
    )
  )

  const mockERC20Address = helloWorldDeploymentData.addresses.mockERC20
  const mockERC20ABI = JSON.parse(
    fs.readFileSync(path.resolve(__dirname, '../abis/ERC20Mock.json'), 'utf8')
  )

  const mockERC20 = new ethers.Contract(mockERC20Address, mockERC20ABI, wallet)

  const amountToMint = ethers.parseUnits('1000', 18)

  try {
    const tx = await mockERC20.mint(await wallet.getAddress(), amountToMint)
    await tx.wait()
    console.log(
      `Successfully minted ${ethers.formatUnits(amountToMint, 18)} tokens to ${await wallet.getAddress()}`
    )
    return true
  } catch (error) {
    console.error('Error in minting tokens:', error)
    return false
  }
}

module.exports = mintMockToken
