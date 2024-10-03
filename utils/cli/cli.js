#!/usr/bin/env node
const { Command } = require('commander');
const { mintMockTokensCommand } = require('./commands/mintMockTokens');
const {
  depositIntoStrategyCommand,
} = require('./commands/depositIntoStrategy');
const { createStrategyCommand } = require('./commands/createStrategyForToken');
const { registerAsOperatorCommand } = require('./commands/registerAsOperator');
const { extractAbisCommand } = require('./commands/extractAbis');

const program = new Command();

program
  .name('eigen-cli')
  .description('CLI for interacting with Eigen contracts')
  .version('1.0.0');

program.addCommand(mintMockTokensCommand);
program.addCommand(depositIntoStrategyCommand);
program.addCommand(createStrategyCommand);
program.addCommand(registerAsOperatorCommand);
program.addCommand(extractAbisCommand);

program.parse(process.argv);
