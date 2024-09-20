const { resolve } = require('path');
const { render } = require('cli-testing-library');
const { spawn } = require('child_process');
require('cli-testing-library/extend-expect');

describe('createStrategyCommand', () => {
  let anvil;

  beforeAll((done) => {
    anvil = spawn('anvil');
    anvil.stdout.on('data', async (data) => {
      if (data.toString().includes('Listening on')) {
        // Execute deploy:core command
        const deployCore = spawn('npm', ['run', 'deploy:core'], { shell: true });
        await new Promise((resolve, reject) => {
          deployCore.on('close', (code) => {
            if (code !== 0) {
              reject(new Error('Failed to deploy core contracts'));
            } else {
              resolve();
            }
          });
        });

        const deployHelloWorld = spawn('npm', ['run', 'deploy:hello-world'], { shell: true });
        await new Promise((resolve, reject) => {
          deployHelloWorld.on('close', (code) => {
            if (code !== 0) {
              reject(new Error('Failed to deploy hello-world contracts'));
            } else {
              resolve();
            }
          });
        });

        console.log('Contracts deployed successfully');
        done();
      }
    });
  });

  afterAll(() => {
    if (anvil) {
      anvil.kill();
    }
  });

  test('should confirm that Anvil is running on the correct port', async () => {
    const { exec } = require('child_process');
    const util = require('util');
    const execAsync = util.promisify(exec);

    try {
      const { stdout } = await execAsync('lsof -i :8545');
      expect(stdout).toContain('anvil');
    } catch (error) {
      if (error.code === 1) {
        throw new Error('Anvil is not running on port 8545');
      } else {
        throw error;
      }
    }
  });

  test('should confirm that contracts are deployed', async () => {
    // TODO:
  });

  test('should successfully create a strategy for a token', async () => {
    const tokenAddress = '0x0987654321098765432109876543210987654321';

    const { findByText } = await render('node', [
      resolve(__dirname, '../utils/createStrategyForToken.js'),
      'create-strategy',
      tokenAddress
    ]);

    const output = await findByText('Strategy created successfully');
    expect(output).toBeInTheConsole();
  }, 30000); 

});
