import {config, helloWorldServiceManagerAddress} from "./config"
import {watchHelloWorldServiceManagerNewTaskCreatedEvent } from "../dist/generated";
import { signAndRespondToTask } from "./signAndRespondToTask";


export const monitorNewTasks = async () => {
    watchHelloWorldServiceManagerNewTaskCreatedEvent(config, {
        address: helloWorldServiceManagerAddress,
        onLogs: async (logs) => {
            for (const log of logs) {
                const { args } = log;
                if (args && args.taskIndex !== undefined && args.task) {
                    const { taskIndex, task } = args;
                    console.log(`New task detected: Hello, ${task.name}`);
                    /// TODO: Clean this up
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