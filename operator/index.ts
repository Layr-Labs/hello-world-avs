import { createConfig, http } from '@wagmi/core'
import { anvil } from '@wagmi/core/chains'
import { createClient, keccak256, toBytes } from 'viem'
import { privateKeyToAccount } from 'viem/accounts'
import { writeDelegationManager, readDelegationManager, readAvsDirectory, watchHelloWorldServiceManagerNewTaskCreatedEvent, writeEcdsaStakeRegistry } from "../dist/generated";
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

// Load core deployment data
const avsDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `../contracts/deployments/hello-world/${chainId}.json`), 'utf8'));
const coreDeploymentData = JSON.parse(fs.readFileSync(path.resolve(__dirname, `../contracts/deployments/core/${chainId}.json`), 'utf8'));

const delegationManagerAddress = coreDeploymentData.addresses.delegation;
const avsDirectoryAddress = coreDeploymentData.addresses.avsDirectory;
const helloWorldServiceManagerAddress = avsDeploymentData.addresses.helloWorldServiceManager;
const stakeRegistryAddress = avsDeploymentData.addresses.stakeRegistry;

const registerOperator = async () => {
    const registered = await readDelegationManager(
        config,
        {
        address: delegationManagerAddress,
        functionName: "isOperator",
        args: [account.address]
        }
    )

    console.log(registered);

    if (!registered){
        await writeDelegationManager(config, {
            account: account,
            address: delegationManagerAddress,
            functionName: "registerAsOperator",
            args: [{
                __deprecated_earningsReceiver: "0x0000000000000000000000000000000000000000",
                delegationApprover: "0x0000000000000000000000000000000000000000",
                stakerOptOutWindowBlocks: 0
            }, ""],
        });
    }
    
    const salt = keccak256(toBytes("random_salt_string"));
    const expiry = BigInt(Math.floor(Date.now() / 1000) + 3600); // 1 hour from now

    // Calculate the registration digest hash
    const digestHash = await readAvsDirectory(config, {
        address: avsDirectoryAddress,
        functionName: "calculateOperatorAVSRegistrationDigestHash",
        args: [account.address, helloWorldServiceManagerAddress, salt, expiry]
    });


    const sig = await account.sign({hash:digestHash});

    await writeEcdsaStakeRegistry(
        config,{    
        account: account,
        address: stakeRegistryAddress,
        functionName: "registerOperatorWithSignature",
        args: [{signature: sig, salt: salt, expiry:expiry}, account.address]
    })

    console.log("Operator registered to AVS");
};

const signAndRespondToTask = async (taskIndex: number, taskCreatedBlock: number, taskName: string) => {
    console.log(
        `Signing and responding to task ${taskIndex}`
    )

    console.log(`Responded to task.`);
};

const monitorNewTasks = async () => {
    const unwatch = watchHelloWorldServiceManagerNewTaskCreatedEvent(config, {
        address: helloWorldServiceManagerAddress,
        onLogs: async (logs) => {
            for (const log of logs) {
                const { args } = log;
                if (args && args.taskIndex !== undefined && args.task) {
                    const { taskIndex, task } = args;
                    console.log(`New task detected: Hello, ${task.name}`);
                    await signAndRespondToTask(
                        taskIndex,
                        task.taskCreatedBlock,
                        task.name
                    );
                }else {
                    console.log("task data not defined")
                }
            }
        },
    });

    console.log("Monitoring for new tasks...");
};

const main = async () => {
    await registerOperator();
    monitorNewTasks().catch((error) => {
        console.error("Error monitoring tasks:", error);
    });
};

main().catch((error) => {
    console.error("Error in main function:", error);
});