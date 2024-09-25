const {
  createStrategyForToken,
} = require('../utils/cli/commands/createStrategyForToken')
const { mintMockTokens } = require('../utils/cli/commands/mintMockTokens')
const {
  depositIntoStrategy,
} = require('../utils/cli/commands/depositIntoStrategy')

const {
  registerAsOperator,
} = require('../utils/cli/commands/registerAsOperator')
require('dotenv').config()

describe('commands', () => {
  describe('createStrategyForToken', () => {
    test('Successfully creates strategy for token', async () => {
      const config = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        tokenAddress: '',
      }

      const [error, result] = await createStrategyForToken(config)

      expect(error).toBeNull()
      expect(result).not.toBeNull()

      const zeroAddress = '0x0000000000000000000000000000000000000000'
      expect(result).not.toBe(zeroAddress)
    })
  })

  describe('mintMockTokens', () => {
    test('Successfully mints mock tokens', async () => {
      const config = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        tokenAddress: '',
        amount: '1000',
      }

      const [error, result] = await mintMockTokens(config)

      expect(error).toBeNull()
      expect(result).not.toBeNull()
    })
  })

  describe('depositIntoStrategy', () => {
    test('Successfully deposits tokens into strategy', async () => {
      const mintConfig = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        amount: '2000',
      }

      const [mintError, mintResult] = await mintMockTokens(mintConfig)

      expect(mintError).toBeNull()
      expect(mintResult).not.toBeNull()

      const depositConfig = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        amount: '1000',
      }

      const [depositError, depositResult] =
        await depositIntoStrategy(depositConfig)

      expect(depositError).toBeNull()
      expect(depositResult).not.toBeNull()
    })
  })

  describe('registerAsOperator', () => {
    test('Successfully registers as an operator', async () => {
      const config = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        operatorDetails: {},
      }

      const [error, result] = await registerAsOperator(config)

      expect(error).toBeNull()
      expect(result).not.toBeNull()
    })
  })
})
