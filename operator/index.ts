import { ethers } from "ethers";
import * as dotenv from "dotenv";
import { delegationABI } from "./abis/delegationABI";
import { contractABI } from './abis/contractABI';
import { registryABI } from './abis/registryABI';
import { avsDirectoryABI } from './abis/avsDirectoryABI';
import { RemoteSigner } from "./remoteSigner";
dotenv.config();

const remoteSignerUrl = process.env.REMOTE_SIGNER_URL!;
const operatorAddress = process.env.OPERATOR_ADDRESS!;
const provider = new ethers.providers.JsonRpcProvider(process.env.RPC_URL);
const wallet = new ethers.Wallet(process.env.PRIVATE_KEY!, provider);
const signer = new RemoteSigner(operatorAddress, provider, remoteSignerUrl);
let address = "";

const delegationManagerAddress = process.env.DELEGATION_MANAGER_ADDRESS!;
const contractAddress = process.env.CONTRACT_ADDRESS!;
const stakeRegistryAddress = process.env.STAKE_REGISTRY_ADDRESS!;
const avsDirectoryAddress = process.env.AVS_DIRECTORY_ADDRESS!;

const signerType = process.env.SIGNER_TYPE!;

let delegationManager: ethers.Contract;
let contract: ethers.Contract;
let registryContract: ethers.Contract;
let avsDirectory: ethers.Contract;

const signAndRespondToTask = async (taskIndex: number, taskCreatedBlock: number, taskName: string) => {
    const message = `Hello, ${taskName}`;
    const messageHash = ethers.utils.solidityKeccak256(["string"], [message]);

    let signature = "";
    if (signerType === "local") {
        console.log("Using local private key to sign message")
        const messageBytes = ethers.utils.arrayify(messageHash);
        signature = await wallet.signMessage(messageBytes);
    } else if (signerType === "remote") {
        console.log("Using remote signer to sign message")
        signature = await signer.signMessage(messageHash);
    }

    console.log(
        `Signing and responding to task ${taskIndex}`
    )

    const tx = await contract.respondToTask(
        { name: taskName, taskCreatedBlock: taskCreatedBlock },
        taskIndex,
        signature
    );
    await tx.wait();
    console.log(`Responded to task.`);
};

const registerOperator = async () => {
    console.log("check")
    const tx1 = await delegationManager.registerAsOperator({
        earningsReceiver: address,
        delegationApprover: "0x0000000000000000000000000000000000000000",
        stakerOptOutWindowBlocks: 0
    }, "");
    await tx1.wait();
    console.log("Operator registered on EL successfully");

    const salt = ethers.utils.hexlify(ethers.utils.randomBytes(32));
    const expiry = Math.floor(Date.now() / 1000) + 3600; // Example expiry, 1 hour from now

    // Define the output structure
    let operatorSignature = {
        expiry: expiry,
        salt: salt,
        signature: ""
    };

    // Calculate the digest hash using the avsDirectory's method
    const digestHash = await avsDirectory.calculateOperatorAVSRegistrationDigestHash(
        address,
        contract.address,
        salt,
        expiry
    );

    // // Sign the digest hash with the operator's private key
    // TODO(shrimalmadhur): I am not completely sure about how to make this work with remote signer
    // as the signDigest function is not available on the remote signer.
    const signingKey = new ethers.utils.SigningKey(process.env.PRIVATE_KEY!);
    const signature = signingKey.signDigest(digestHash);

    // // Encode the signature in the required format
    operatorSignature.signature = ethers.utils.joinSignature(signature);

    const tx2 = await registryContract.registerOperatorWithSignature(
        operatorSignature,
        address
    );
    await tx2.wait();
    console.log("Operator registered on AVS successfully");
};

const monitorNewTasks = async () => {
    await contract.createNewTask("EigenWorld");

    contract.on("NewTaskCreated", async (taskIndex: number, task: any) => {
        console.log(`New task detected: Hello, ${task.name}`);
        await signAndRespondToTask(taskIndex, task.taskCreatedBlock, task.name);
    });

    console.log("Monitoring for new tasks...");
};

const main = async () => {
    if (signerType === "local") {
        address = wallet.address;
        delegationManager = new ethers.Contract(delegationManagerAddress, delegationABI, wallet);
        contract = new ethers.Contract(contractAddress, contractABI, wallet);
        registryContract = new ethers.Contract(stakeRegistryAddress, registryABI, wallet);
        avsDirectory = new ethers.Contract(avsDirectoryAddress, avsDirectoryABI, wallet);
    } else {
        address = await signer.getAddress();
        delegationManager = new ethers.Contract(delegationManagerAddress, delegationABI, signer);
        contract = new ethers.Contract(contractAddress, contractABI, signer);
        registryContract = new ethers.Contract(stakeRegistryAddress, registryABI, signer);
        avsDirectory = new ethers.Contract(avsDirectoryAddress, avsDirectoryABI, signer);
    }
    await registerOperator();
    monitorNewTasks().catch((error) => {
        console.error("Error monitoring tasks:", error);
    });
};

main().catch((error) => {
    console.error("Error in main function:", error);
});

interface JsonRpcRequest {
    jsonrpc: string;
    method: string;
    params: any[];
    id: number;
}

interface JsonRpcResponse {
    jsonrpc: string;
    result?: any;
    error?: {
        code: number;
        message: string;
    };
    id: number;
}

async function callJsonRpcEndpoint(
    url: string,
    method: string,
    params: any[] = []
): Promise<any> {
    const request: JsonRpcRequest = {
        jsonrpc: "2.0",
        method,
        params,
        id: 1, // You might want to generate a unique id for each request
    };

    const response = await fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(request),
    });

    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    const jsonResponse: JsonRpcResponse = await response.json();

    if (jsonResponse.error) {
        throw new Error(`JSON-RPC error: ${jsonResponse.error.message}`);
    }

    return jsonResponse.result;
}