const { describe, it, expect } = require('@jest/globals');
const {
  createStrategyForToken,
} = require('../utils/cli/commands/createStrategyForToken');
const { mintMockTokens } = require('../utils/cli/commands/mintMockTokens');
const {
  depositIntoStrategy,
} = require('../utils/cli/commands/depositIntoStrategy');
const {
  registerAsOperator,
} = require('../utils/cli/commands/registerAsOperator');
const { extractAbis } = require('../utils/cli/commands/extractAbis');
const fs = require('fs');
const path = require('path');
require('dotenv').config();

describe('commands', () => {
  describe('createStrategyForToken', () => {
    it('Successfully creates strategy for token', async () => {
      const config = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        tokenAddress: '',
      };

      const [error, result] = await createStrategyForToken(config);

      expect(error).toBeNull();
      expect(result).not.toBeNull();

      const zeroAddress = '0x0000000000000000000000000000000000000000';
      expect(result).not.toBe(zeroAddress);
    });
  });

  describe('mintMockTokens', () => {
    it('Successfully mints mock tokens', async () => {
      const config = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        tokenAddress: '',
        amount: '1000',
      };

      const [error, result] = await mintMockTokens(config);

      expect(error).toBeNull();
      expect(result).not.toBeNull();
    });
  });

  describe('depositIntoStrategy', () => {
    it('Successfully deposits tokens into strategy', async () => {
      const mintConfig = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        amount: '2000',
      };

      const [mintError, mintResult] = await mintMockTokens(mintConfig);

      expect(mintError).toBeNull();
      expect(mintResult).not.toBeNull();

      const depositConfig = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        amount: '1000',
      };

      const [depositError, depositResult] =
        await depositIntoStrategy(depositConfig);

      expect(depositError).toBeNull();
      expect(depositResult).not.toBeNull();
    });
  });

  describe('registerAsOperator', () => {
    it('Successfully registers as an operator', async () => {
      const config = {
        rpcUrl: 'http://localhost:8545',
        privateKey: process.env.PRIVATE_KEY,
        operatorDetails: {},
      };

      const [error, result] = await registerAsOperator(config);

      expect(error).toBeNull();
      expect(result).not.toBeNull();
    });
  });

  describe('extractAbis', () => {
    it('Successfully extracts ABIs', async () => {
      const testArtifactsDir = path.join(__dirname, 'test-artifacts');
      const testAbiDir = path.join(__dirname, 'test-abis');
      const testContractName = 'TestContract';

      // Create test artifacts directory and file
      fs.mkdirSync(testArtifactsDir, { recursive: true });
      fs.mkdirSync(path.join(testArtifactsDir, `${testContractName}.sol`), {
        recursive: true,
      });
      fs.writeFileSync(
        path.join(
          testArtifactsDir,
          `${testContractName}.sol`,
          `${testContractName}.json`,
        ),
        JSON.stringify({ abi: [{ type: 'function', name: 'testFunction' }] }),
      );

      const options = {
        dir: testArtifactsDir,
        output: testAbiDir,
        contracts: [testContractName],
      };

      const [error, result] = await extractAbis(options);

      expect(error).toBeNull();
      expect(result).not.toBeNull();
      expect(result).toContain(`Extracted ABI for ${testContractName}`);

      // Check if ABI file was created
      const abiFilePath = path.join(testAbiDir, `${testContractName}.json`);
      expect(fs.existsSync(abiFilePath)).toBe(true);

      // Clean up test directories
      fs.rmSync(testArtifactsDir, { recursive: true, force: true });
      fs.rmSync(testAbiDir, { recursive: true, force: true });
    });
  });
});
