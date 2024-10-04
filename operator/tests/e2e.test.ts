import { createAnvil, Anvil } from "@viem/anvil";
import { describe, beforeAll, afterAll, beforeEach, it, afterEach, expect } from 'vitest'
import { exec } from 'child_process';
import util from 'util';
import fs from 'fs';
import path from 'path';
import {account} from "../config"
import { createPublicClient, createWalletClient } from 'viem';
import { anvil } from '@wagmi/core/chains'
import {http } from 'viem'

const execPromise = util.promisify(exec);

describe('signAndRespondToTask', () => {
  let anvilInstance: Anvil;

  const readClient = createPublicClient({ 
  chain: anvil,
  transport: http()
})

  const writeClient = createWalletClient({ 
  account: account,
  chain: anvil,
  transport: http()
})

  beforeAll(async () => {
    const anvil = createAnvil({ dumpState: "./operator/tests/anvilState.json" });
    await anvil.start();
    await execPromise('npm run deploy:core');
    await execPromise('npm run deploy:hello-world');
    await anvil.stop();
  });

  beforeEach(async () => {
    anvilInstance = createAnvil({ loadState: "./operator/tests/anvilState.json" });
    await anvilInstance.start();
  });

  it('confirm contract state deployed', async () => {
    const networkId = 31337;
    const deploymentPath = path.join(__dirname, '../../', 'contracts', 'deployments', 'hello-world', `${networkId}.json`);
    const deploymentData = JSON.parse(fs.readFileSync(deploymentPath, 'utf8'));

    for (const [, address] of Object.entries(deploymentData.addresses)) {
        const result = await readClient.getCode({ address: address as `0x${string}` });
        expect(result).not.toBe('0x');
    }
  });

  afterEach(async () => {
    await anvilInstance.stop();
  });

  afterAll(async () => {
    await anvilInstance.stop();
  });

});