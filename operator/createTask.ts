import { writeHelloWorldServiceManager } from "../dist/generated";
import {config, account, helloWorldServiceManagerAddress } from "./config"

export async function createNewTask(taskName: string) {
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