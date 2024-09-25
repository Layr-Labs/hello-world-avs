import { ethers } from 'ethers';
import * as dotenv from 'dotenv';
const fs = require('fs');
const path = require('path');
dotenv.config();

// Import util modules
const registerAsOperator = require('../utils/registerAsOperator');
const registerOperatorWithAVS = require('../utils/registerOperatorWithAVS');

// Check if the process.env object is empty
if (!Object.keys(process.env).length) {
  throw new Error('process.env object is empty');
}

// Setup env variables
const provider = new ethers.JsonRpcProvider(process.env.RPC_URL);

const wallet = new ethers.Wallet(process.env.PRIVATE_KEY || '', provider);
/// TODO: Hack
let chainId = 31337;

const avsDeploymentData = JSON.parse(
  fs.readFileSync(
    path.resolve(
      __dirname,
      `../contracts/deployments/hello-world/${chainId}.json`,
    ),
    'utf8',
  ),
);

const helloWorldServiceManagerAddress =
  avsDeploymentData.addresses.helloWorldServiceManager;
const helloWorldServiceManagerABI = JSON.parse(
  fs.readFileSync(
    path.resolve(__dirname, '../abis/HelloWorldServiceManager.json'),
    'utf8',
  ),
);
const helloWorldServiceManager = new ethers.Contract(
  helloWorldServiceManagerAddress,
  helloWorldServiceManagerABI,
  wallet,
);

const signAndRespondToTask = async (
  taskIndex: number,
  taskCreatedBlock: number,
  taskName: string,
) => {
  const message = `Hello, ${taskName}`;
  const messageHash = ethers.solidityPackedKeccak256(['string'], [message]);
  const messageBytes = ethers.getBytes(messageHash);
  const signature = await wallet.signMessage(messageBytes);

  console.log(`Signing and responding to task ${taskIndex}`);

  const tx = await helloWorldServiceManager.respondToTask(
    { name: taskName, taskCreatedBlock: taskCreatedBlock },
    taskIndex,
    signature,
  );
  await tx.wait();
  console.log(`Responded to task.`);
};

const monitorNewTasks = async () => {
  //console.log(`Creating new task "EigenWorld"`);
  //await helloWorldServiceManager.createNewTask("EigenWorld");

  helloWorldServiceManager.on(
    'NewTaskCreated',
    async (taskIndex: number, task: any) => {
      console.log(`New task detected: Hello, ${task.name}`);
      await signAndRespondToTask(taskIndex, task.taskCreatedBlock, task.name);
    },
  );

  console.log('Monitoring for new tasks...');
};

const main = async () => {
  /// TODO: Check that the operator is registered with EigenLayer
  /// TODO: Check that strategy exists
  /// TODO: Check that operator has tokens deposited in strategy
  /// TODO: Check that the operator is registered with the AVS
  monitorNewTasks().catch((error) => {
    console.error('Error monitoring tasks:', error);
  });
};

main().catch((error) => {
  console.error('Error in main function:', error);
});
