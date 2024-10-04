import { writeDelegationManager, readDelegationManager } from "../dist/generated";
import {config, delegationManagerAddress, account } from "./config"

export const registerOperator = async () => {
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
    


    console.log("Operator registered to AVS");
};
