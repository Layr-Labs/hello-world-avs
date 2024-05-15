"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const ethers_1 = require("ethers");
const dotenv = __importStar(require("dotenv"));
dotenv.config();
const provider = new ethers_1.ethers.JsonRpcProvider(process.env.RPC_URL);
const wallet = new ethers_1.ethers.Wallet(process.env.PRIVATE_KEY, provider);
const contractAddress = process.env.CONTRACT_ADDRESS;
const contractABI = [{ "type": "constructor", "inputs": [{ "name": "_avsDirectory", "type": "address", "internalType": "contract IAVSDirectory" }, { "name": "_registryCoordinator", "type": "address", "internalType": "contract IRegistryCoordinator" }, { "name": "_stakeRegistry", "type": "address", "internalType": "contract IStakeRegistry" }], "stateMutability": "nonpayable" }, { "type": "function", "name": "allTaskHashes", "inputs": [{ "name": "", "type": "uint32", "internalType": "uint32" }], "outputs": [{ "name": "", "type": "bytes32", "internalType": "bytes32" }], "stateMutability": "view" }, { "type": "function", "name": "allTaskResponses", "inputs": [{ "name": "", "type": "address", "internalType": "address" }, { "name": "", "type": "uint32", "internalType": "uint32" }], "outputs": [{ "name": "", "type": "bytes", "internalType": "bytes" }], "stateMutability": "view" }, { "type": "function", "name": "avsDirectory", "inputs": [], "outputs": [{ "name": "", "type": "address", "internalType": "address" }], "stateMutability": "view" }, { "type": "function", "name": "createNewTask", "inputs": [{ "name": "name", "type": "string", "internalType": "string" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "deregisterOperatorFromAVS", "inputs": [{ "name": "operator", "type": "address", "internalType": "address" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "getOperatorRestakedStrategies", "inputs": [{ "name": "operator", "type": "address", "internalType": "address" }], "outputs": [{ "name": "", "type": "address[]", "internalType": "address[]" }], "stateMutability": "view" }, { "type": "function", "name": "getRestakeableStrategies", "inputs": [], "outputs": [{ "name": "", "type": "address[]", "internalType": "address[]" }], "stateMutability": "view" }, { "type": "function", "name": "latestTaskNum", "inputs": [], "outputs": [{ "name": "", "type": "uint32", "internalType": "uint32" }], "stateMutability": "view" }, { "type": "function", "name": "owner", "inputs": [], "outputs": [{ "name": "", "type": "address", "internalType": "address" }], "stateMutability": "view" }, { "type": "function", "name": "pause", "inputs": [{ "name": "newPausedStatus", "type": "uint256", "internalType": "uint256" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "pauseAll", "inputs": [], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "paused", "inputs": [{ "name": "index", "type": "uint8", "internalType": "uint8" }], "outputs": [{ "name": "", "type": "bool", "internalType": "bool" }], "stateMutability": "view" }, { "type": "function", "name": "paused", "inputs": [], "outputs": [{ "name": "", "type": "uint256", "internalType": "uint256" }], "stateMutability": "view" }, { "type": "function", "name": "pauserRegistry", "inputs": [], "outputs": [{ "name": "", "type": "address", "internalType": "contract IPauserRegistry" }], "stateMutability": "view" }, { "type": "function", "name": "payForRange", "inputs": [{ "name": "rangePayments", "type": "tuple[]", "internalType": "struct IPaymentCoordinator.RangePayment[]", "components": [{ "name": "strategiesAndMultipliers", "type": "tuple[]", "internalType": "struct IPaymentCoordinator.StrategyAndMultiplier[]", "components": [{ "name": "strategy", "type": "address", "internalType": "contract IStrategy" }, { "name": "multiplier", "type": "uint96", "internalType": "uint96" }] }, { "name": "token", "type": "address", "internalType": "contract IERC20" }, { "name": "amount", "type": "uint256", "internalType": "uint256" }, { "name": "startTimestamp", "type": "uint32", "internalType": "uint32" }, { "name": "duration", "type": "uint32", "internalType": "uint32" }] }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "registerOperatorToAVS", "inputs": [{ "name": "operator", "type": "address", "internalType": "address" }, { "name": "operatorSignature", "type": "tuple", "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry", "components": [{ "name": "signature", "type": "bytes", "internalType": "bytes" }, { "name": "salt", "type": "bytes32", "internalType": "bytes32" }, { "name": "expiry", "type": "uint256", "internalType": "uint256" }] }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "registryCoordinator", "inputs": [], "outputs": [{ "name": "", "type": "address", "internalType": "contract IRegistryCoordinator" }], "stateMutability": "view" }, { "type": "function", "name": "renounceOwnership", "inputs": [], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "respondToTask", "inputs": [{ "name": "task", "type": "tuple", "internalType": "struct IHelloWorldServiceManager.Task", "components": [{ "name": "name", "type": "string", "internalType": "string" }, { "name": "taskCreatedBlock", "type": "uint32", "internalType": "uint32" }] }, { "name": "referenceTaskIndex", "type": "uint32", "internalType": "uint32" }, { "name": "signature", "type": "bytes", "internalType": "bytes" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "setPauserRegistry", "inputs": [{ "name": "newPauserRegistry", "type": "address", "internalType": "contract IPauserRegistry" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "transferOwnership", "inputs": [{ "name": "newOwner", "type": "address", "internalType": "address" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "unpause", "inputs": [{ "name": "newPausedStatus", "type": "uint256", "internalType": "uint256" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "function", "name": "updateAVSMetadataURI", "inputs": [{ "name": "_metadataURI", "type": "string", "internalType": "string" }], "outputs": [], "stateMutability": "nonpayable" }, { "type": "event", "name": "Initialized", "inputs": [{ "name": "version", "type": "uint8", "indexed": false, "internalType": "uint8" }], "anonymous": false }, { "type": "event", "name": "NewTaskCreated", "inputs": [{ "name": "taskIndex", "type": "uint32", "indexed": true, "internalType": "uint32" }, { "name": "task", "type": "tuple", "indexed": false, "internalType": "struct IHelloWorldServiceManager.Task", "components": [{ "name": "name", "type": "string", "internalType": "string" }, { "name": "taskCreatedBlock", "type": "uint32", "internalType": "uint32" }] }], "anonymous": false }, { "type": "event", "name": "OwnershipTransferred", "inputs": [{ "name": "previousOwner", "type": "address", "indexed": true, "internalType": "address" }, { "name": "newOwner", "type": "address", "indexed": true, "internalType": "address" }], "anonymous": false }, { "type": "event", "name": "Paused", "inputs": [{ "name": "account", "type": "address", "indexed": true, "internalType": "address" }, { "name": "newPausedStatus", "type": "uint256", "indexed": false, "internalType": "uint256" }], "anonymous": false }, { "type": "event", "name": "PauserRegistrySet", "inputs": [{ "name": "pauserRegistry", "type": "address", "indexed": false, "internalType": "contract IPauserRegistry" }, { "name": "newPauserRegistry", "type": "address", "indexed": false, "internalType": "contract IPauserRegistry" }], "anonymous": false }, { "type": "event", "name": "TaskResponded", "inputs": [{ "name": "taskIndex", "type": "uint32", "indexed": true, "internalType": "uint32" }, { "name": "task", "type": "tuple", "indexed": false, "internalType": "struct IHelloWorldServiceManager.Task", "components": [{ "name": "name", "type": "string", "internalType": "string" }, { "name": "taskCreatedBlock", "type": "uint32", "internalType": "uint32" }] }, { "name": "operator", "type": "address", "indexed": false, "internalType": "address" }], "anonymous": false }, { "type": "event", "name": "Unpaused", "inputs": [{ "name": "account", "type": "address", "indexed": true, "internalType": "address" }, { "name": "newPausedStatus", "type": "uint256", "indexed": false, "internalType": "uint256" }], "anonymous": false }];
const contract = new ethers_1.ethers.Contract(contractAddress, contractABI, wallet);
const signAndRespondToTask = (taskIndex, taskName) => __awaiter(void 0, void 0, void 0, function* () {
    const message = `Hello, ${taskName}`;
    const messageHash = ethers_1.ethers.solidityPackedKeccak256(["string"], [message]);
    const messageBytes = ethers_1.ethers.toBeArray(messageHash);
    const signature = yield wallet.signMessage(messageBytes);
    const tx = yield contract.respondToTask({ name: taskName, taskCreatedBlock: taskIndex }, taskIndex, signature);
    yield tx.wait();
    console.log(`Responded to task ${taskIndex} with signature ${signature}`);
});
const monitorNewTasks = () => __awaiter(void 0, void 0, void 0, function* () {
    contract.on("NewTaskCreated", (taskIndex, task) => __awaiter(void 0, void 0, void 0, function* () {
        console.log(`New task created: ${taskIndex}`, task);
        yield signAndRespondToTask(taskIndex, task.name);
    }));
    console.log("Monitoring for new tasks...");
});
monitorNewTasks().catch((error) => {
    console.error("Error monitoring tasks:", error);
});
