import { createConfig, http } from '@wagmi/core'
import { anvil } from '@wagmi/core/chains'
import { createClient } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { writeHelloWorldServiceManager } from "../dist/generated";

import * as dotenv from "dotenv";
import path from 'path';
import fs from 'fs';
dotenv.config();

// Check if the process.env object is empty
if (!Object.keys(process.env).length) {
    throw new Error("process.env object is empty");
}

const account = privateKeyToAccount(process.env.PRIVATE_KEY as `0x${string}`)

const config = createConfig({
    chains: [anvil],
    client({ chain }) {
        return createClient({ 
            chain, transport: http() })
    },
})

const chainId = 31337

const __dirname = process.cwd();

// Load core deployment data
const avsDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `./contracts/deployments/hello-world/${chainId}.json`), 'utf8'));

const helloWorldServiceManagerAddress = avsDeploymentData.addresses.helloWorldServiceManager;

// Function to generate random names
function generateRandomName(): string {
    const adjectives = ['Quick', 'Lazy', 'Sleepy', 'Noisy', 'Hungry'];
    const nouns = ['Fox', 'Dog', 'Cat', 'Mouse', 'Bear'];
    const adjective = adjectives[Math.floor(Math.random() * adjectives.length)];
    const noun = nouns[Math.floor(Math.random() * nouns.length)];
    const randomName = `${adjective}${noun}${Math.floor(Math.random() * 1000)}`;
    return randomName;
}

async function createNewTask(taskName: string) {
    try {
        const result = await writeHelloWorldServiceManager(config, {
            account: account,
            address: helloWorldServiceManagerAddress,
            functionName: "createNewTask",
            args: [taskName]
        });
        
        console.log(`Transaction successful with hash: ${result}`);
    } catch (error) {
        console.error('Error sending transaction:', error);
    }
}

// Function to create a new task with a random name every 15 seconds
function startCreatingTasks() {
    setInterval(() => {
        const randomName = generateRandomName();
        console.log(`Creating new task with name: ${randomName}`);
        createNewTask(randomName);
    }, 5000);
}

// Start the process
startCreatingTasks();