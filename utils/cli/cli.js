#!/usr/bin/env node
const { program } = require('commander')
const createStrategyCommand = require('./commands/createStrategyForToken')

program.addCommand(createStrategyCommand)

if (!process.argv.slice(2).length) {
  program.outputHelp()
}

program.parse(process.argv)
