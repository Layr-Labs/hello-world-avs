import { verifyMessage, signatureToCompactSignature} from 'viem'
import { writeHelloWorldServiceManager, readEcdsaStakeRegistry } from "../dist/generated";
import {config, account, helloWorldServiceManagerAddress, stakeRegistryAddress } from "./config"

export const signAndRespondToTask = async (taskIndex: number, taskCreatedBlock: number, taskName: string) => {
    console.log(
        `Signing and responding to task ${taskIndex}`
    )

    try {
        const task = {
            name: taskName,
            taskCreatedBlock: taskCreatedBlock
        };

        const message = `Hello, ${taskName}`;

        const signature = await account.signMessage({
            message: message 
        });

        const result = await writeHelloWorldServiceManager(config, {
            account: account,
            address: helloWorldServiceManagerAddress,
            functionName: "respondToTask",
            args: [task, taskIndex, signature]
        });
        console.log(`Transaction successful with hash: ${result}`);
    } catch (error) {
        // console.error('Error responding to task:', error);
    }
};