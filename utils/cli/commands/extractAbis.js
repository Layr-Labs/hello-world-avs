const { Command } = require('commander');
const fs = require('fs');
const path = require('path');

const extractAbisCommand = new Command('extract-abis')
  .description('Extract ABIs from contract artifacts')
  .option('-d, --dir <directory>', 'Artifacts directory', 'contracts/out')
  .option('-o, --output <directory>', 'Output directory for ABIs', 'abis')
  .option('-c, --contracts <contracts...>', 'Contracts to extract ABIs from')
  .action(async (options) => {
    const [error, result] = await extractAbis(options);
    if (error) {
      console.error('Error extracting ABIs:', error.message);
      process.exit(1);
    }
    console.log(result);
  });

async function extractAbis(options) {
  const {
    dir: artifactsDir,
    output: abiDir,
    contracts: contractsToExtract,
  } = options;

  if (!fs.existsSync(abiDir)) {
    fs.mkdirSync(abiDir, { recursive: true });
  }

  const [checkError] = checkArtifactsDirectory(artifactsDir);
  if (checkError) {
    return [checkError, null];
  }

  const results = [];
  for (const contractName of contractsToExtract) {
    const [extractError, extractResult] = await extractAbi(
      contractName,
      artifactsDir,
      abiDir,
    );
    if (extractError) {
      results.push(
        `Error extracting ABI for ${contractName}: ${extractError.message}`,
      );
    } else {
      results.push(extractResult);
    }
  }

  return [null, results.join('\n')];
}

function checkArtifactsDirectory(artifactsDir) {
  if (!fs.existsSync(artifactsDir)) {
    return [
      new Error(`The artifacts directory '${artifactsDir}' does not exist.`),
      null,
    ];
  }

  const files = fs.readdirSync(artifactsDir);
  if (files.length === 0) {
    return [
      new Error(`The artifacts directory '${artifactsDir}' is empty.`),
      null,
    ];
  }

  return [null, true];
}

async function extractAbi(contractName, artifactsDir, abiDir) {
  const outputPath = path.join(
    artifactsDir,
    `${contractName}.sol`,
    `${contractName}.json`,
  );
  const abiOutputPath = path.join(abiDir, `${contractName}.json`);

  try {
    const contractData = JSON.parse(fs.readFileSync(outputPath, 'utf8'));
    const abi = JSON.stringify(contractData.abi);
    fs.writeFileSync(abiOutputPath, abi);
    return [null, `Extracted ABI for ${contractName}`];
  } catch (error) {
    return [error, null];
  }
}

module.exports = { extractAbisCommand, extractAbis };
