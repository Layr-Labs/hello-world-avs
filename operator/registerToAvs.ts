import { keccak256, toBytes } from 'viem'
import { readAvsDirectory, readEcdsaStakeRegistry, writeEcdsaStakeRegistry } from "../dist/generated";
import {config, account, avsDirectoryAddress, helloWorldServiceManagerAddress, stakeRegistryAddress} from "./config"

export const registerToAvs = async () => {
    // Check if the operator is already registered
    const isRegistered = await readEcdsaStakeRegistry(config, {
        address: stakeRegistryAddress,
        functionName: "operatorRegistered",
        args: [account.address]
    });

    if (!isRegistered) {
        const salt = keccak256(toBytes("random_salt_string"));
        const expiry = BigInt(Math.floor(Date.now() / 1000) + 3600); // 1 hour from now

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

        console.log(`Operator ${account.address} registered successfully.`);
    } else {
        console.log(`Operator ${account.address} is already registered.`);
    }
}