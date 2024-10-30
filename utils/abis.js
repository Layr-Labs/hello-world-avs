const fs = require('fs');
const path = require('path');

const abiDir = 'abis';
const contractsDir = 'contracts';
const artifactsDir = path.join(contractsDir, 'out');

const contractsToExtract = [
  'IAVSDirectory',
  'IDelegationManager',
  'ECDSAStakeRegistry',
  'HelloWorldServiceManager'
];

if (!fs.existsSync(abiDir)) {
  fs.mkdirSync(abiDir);
}

function checkArtifactsDirectory() {
  if (!fs.existsSync(artifactsDir)) {
    console.error(`The artifacts directory '${artifactsDir}' does not exist.`);
    console.log('Please compile your contracts first using "forge build"');
    process.exit(1);
  }

  const files = fs.readdirSync(artifactsDir);
  if (files.length === 0) {
    console.error(`The artifacts directory '${artifactsDir}' is empty.`);
    console.log('Please compile your contracts first using "forge build" or confirm the path is correct.');
    process.exit(1);
  }
}

function extractAbi(contractName) {
  const outputPath = path.join(artifactsDir, `${contractName}.sol`, `${contractName}.json`);
  const abiOutputPath = path.join(abiDir, `${contractName}.json`);

  try {
    const contractData = JSON.parse(fs.readFileSync(outputPath, 'utf8'));
    const abi = JSON.stringify(contractData.abi, null, 2);
    fs.writeFileSync(abiOutputPath, abi);
    console.log(`Extracted ABI for ${contractName}`);
  } catch (error) {
    console.error(`Error extracting ABI for ${contractName}:`, error.message);
  }
}

checkArtifactsDirectory();

for (const contractName of contractsToExtract) {
  extractAbi(contractName);
}

console.log('ABI extraction complete. Check the "abis" directory for the output.');