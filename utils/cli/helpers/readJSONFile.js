const fs = require('fs');

function readJSONFile(filepath) {
  if (!fs.existsSync(filepath)) {
    return ['File not found', null];
  }

  try {
    const fileContent = fs.readFileSync(filepath, 'utf8');
    const jsonData = JSON.parse(fileContent);
    return [null, jsonData];
  } catch (error) {
    return [error.message, null];
  }
}

module.exports = readJSONFile;
