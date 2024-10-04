import { registerOperator } from "./registerAsOperator";
import { registerToAvs } from "./registerToAvs";
import { monitorNewTasks } from "./monitorTasks";

const main = async () => {
    await registerOperator();
    await registerToAvs();
    monitorNewTasks().catch((error) => {
        console.error("Error monitoring tasks:", error);
    });
};

main().catch((error) => {
    console.error("Error in main function:", error);
});
