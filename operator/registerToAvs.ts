import { keccak256, toBytes } from 'viem'
import { readAvsDirectory, writeEcdsaStakeRegistry } from "../dist/generated";
import {config, account, avsDirectoryAddress, helloWorldServiceManagerAddress, stakeRegistryAddress} from "./config"

export const registerToAvs = async () => {
    //// TODO:  check if they're already registered to the avs stake registry
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
}