import type { Config } from '@jest/types';

const config: Config.InitialOptions = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  testMatch: ['**/*.test.ts'],
  verbose: true,
  testTimeout: 60000, // 60 seconds
  maxWorkers: 1, // Run tests sequentially
};

export default config;
