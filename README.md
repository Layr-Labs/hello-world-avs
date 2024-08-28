# Hello World AVS

Welcome to the Hello World AVS.

This project shows you the simplest functionality you can expect from an AVS. It will give you a concrete understanding of the basic components.

![hello-world-png](./assets/hello-world-diagram.png)

AVS User Flow
- AVS consumer requests a "Hello World" message to be generated and signed.
- AVS contract takes on the request and emits an event for the request.
- Any operator who is staked to serve this AVS takes this request, generates this message and signs it.
- The operator submits this message with their signature back to the AVS.
- *If the operator is in fact registered to the AVS and has the minimum needed stake*, the submission is accepted.

That's it. This simple flow highlights some of the core mechanics of how AVSs work.


## Local Devnet Deployment

The following instructions explain how to manually deploy the AVS from scratch including EigenLayer and AVS specific contracts using Foundry (forge) to a local anvil chain, and start Typescript Operator application and tasks.

Install dependencies:
* [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
* [Foundry](https://getfoundry.sh/)
* [tcs](https://www.npmjs.com/package/tcs#installation)
* [ethers](https://www.npmjs.com/package/ethers)

In terminal window #1, execute the following commands:
```sh
# Start local anvil chain
anvil
```

Open a separate terminal window #2, execute the following commands

```sh
# Setup .env file
cp .env.local .env

# Deploy the EigenLayer contracts
(cd contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts &&\
forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast -vvv --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json && \
rm -rf script/output/devnet/M2_from_scratch_deployment_data.json)

# Write the contract addresses to .env
DELEGATION_MANAGER_ADDRESS=$(jq '.transactions[] | select(.contractName == "DelegationManager") | .contractAddress' contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts/broadcast/M2_Deploy_From_Scratch.s.sol/31337/run-latest.json)
echo DELEGATION_MANAGER_ADDRESS=$DELEGATION_MANAGER_ADDRESS | sed 's/"//g' >> .env

AVS_DIRECTORY_ADDRESS=$(jq '.transactions[] | select(.contractName == "AVSDirectory") | .contractAddress' contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts/broadcast/M2_Deploy_From_Scratch.s.sol/31337/run-latest.json)
echo AVS_DIRECTORY_ADDRESS=$AVS_DIRECTORY_ADDRESS | sed 's/"//g' >> .env



# Deploy Hello World AVS specific contracts
(cd contracts &&\
forge script script/HelloWorldDeployer.s.sol --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast debug)


ECDSA_STAKE_REGISTRY_ADDRESS=$(jq '.transactions[] | select(.contractName == "ECDSAStakeRegistry") | .contractAddress' contracts/broadcast/HelloWorldDeployer.s.sol/31337/run-latest.json)
echo ECDSA_STAKE_REGISTRY_ADDRESS=$ECDSA_STAKE_REGISTRY_ADDRESS | sed 's/"//g' >> .env

HELLO_WORLD_SERVICE_MANAGER_ADDRESS=$(jq '.transactions[] | select(.contractName == "HelloWorldServiceManager") | .contractAddress' contracts/broadcast/HelloWorldDeployer.s.sol/31337/run-latest.json)
echo HELLO_WORLD_SERVICE_MANAGER_ADDRESS=$HELLO_WORLD_SERVICE_MANAGER_ADDRESS | sed 's/"//g' >> .env

# Parse and save newly built ABIs for Operator
jq .abi contracts/out/DelegationManager.sol/DelegationManager.json > abis/DelegationManager.abi
jq .abi contracts/out/HelloWorldServiceManager.sol/HelloWorldServiceManager.json > abis/HelloWorldServiceManager.abi
jq .abi contracts/out/ECDSAStakeRegistry.sol/ECDSAStakeRegistry.json > abis/ECDSAStakeRegistry.abi
jq .abi contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts/out/AVSDirectory.sol/AVSDirectory.json > abis/AVSDirectory.abi


# Install Operator package dependencies
npm install

# Start the Operator application
tsc && node dist/index.js



# todo remove this
DATA=0x3d5611f60000000000000000000000000000000000000000000000000000000000000040000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb922660000000000000000000000000000000000000000000000000000000000000060bf68a1ee4cb28b950f7d9f839689439ac650ae275309f75a82901ad63c2025770000000000000000000000000000000000000000000000000000000066cf34560000000000000000000000000000000000000000000000000000000000000041aab8ad201b3b118cf92197e030f8054138966715bbf21478467b370b5c89a2a918b843536b9b062fa69eacb34f72f91719fb74fd9a105635e7971d557e7596591b00000000000000000000000000000000000000000000000000000000000000'
cast send --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 0x9E545E3C0baAB3E08CdfD552C960A1050f373042 $DATA 



```

Open a separate terminal window #3, execute the following commands

```sh
source .env
# Start the createNewTasks application 
tsc && node dist/createNewTasks.js
```



## Holesky Deployment

To deploy the Hello World AVS contracts to the Holesky network, follow these steps:

1. Ensure you have the necessary RPC URL and private key for the Holesky network.
2. Run the deployment script using Foundry:
    ```bash
    # todo test
    forge script script/HelloWorldDeployer.s.sol:HelloWorldDeployer --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast debug
    ```
    Replace `$RPC_URL` with your Holesky RPC URL and `$PRIVATE_KEY` with your private key.

## Deployment on Tenderly Virtual Testnet

Follow the [Tenderly Virtual Testnet Setup Instructions](https://docs.tenderly.co/virtual-testnets/quickstart) to create a new virtual testnet.

Run the following commands:

```sh

# todo: add instructions to create a wallet for testnet account and set private key in .env holesky vars

# Set env vars
cd contracts
source ../.env

# Fund account using the tenderly rpc
curl $TENDERLY_RPC_ADMIN \
-X POST \
-H "Content-Type: application/json" \
-d '{
    "jsonrpc": "2.0",
    "method": "tenderly_setBalance",
    "params": [
      [
        "'"${PUBLIC_KEY}"'"
        ],
      "0xDE0B6B3A7640000"
      ],
    "id": "1234"
}'

# Deploy AVS contracts to Tenderly Holesky using Foundry
forge script script/HelloWorldDeployerHolesky.s.sol:HelloWorldDeployerHolesky \
    --rpc-url $TENDERLY_RPC_ADMIN --private-key $TENDERLY_PRIVATE_KEY --broadcast -vvv debug



```

# Appendix


## Rust Operator instructions

### Automated deployment (uses existing state file)

1. Run `make start-chain-with-contracts-deployed`
    * This will build the contracts, start an Anvil chain, deploy the contracts to it, and leaves the chain running in the current terminal

2. Run `make start-rust-operator`

3. Run `make spam-rust-tasks`

Tests are supported in anvil only . Make sure to run the 1st command before running the  tests:

```
cargo test --workspace
```


## Existing Holesky Testnet Deployment

| Contract Name               | Holesky Address                                   |
| -------------               | -------------                                     |
| Hello World Service Manager | [0x3361953F4a9628672dCBcDb29e91735fb1985390](https://holesky.etherscan.io/address/0x3361953F4a9628672dCBcDb29e91735fb1985390)    |

Please see [Current Testnet Deployment](https://github.com/Layr-Labs/eigenlayer-contracts?tab=readme-ov-file#current-testnet-deployment) for additional deployed addresses.

You don't need to run a deployment script for holesky testnet, the contracts are already deployed.

1. Use the HOLESKY_ namespace env parameters in the code , instead of normal parameters.

2. Run `make start-rust-operator`

3. Run `make spam-rust-tasks `


## Adding a New Strategy

To add a new strategy to the Hello World AVS, follow the guide provided in [`AddNewStrategy.md`](https://github.com/Layr-Labs/hello-world-avs/blob/master/AddNewStrategy.md). This guide walks you through the necessary steps to add and whitelist a new strategy for the AVS.

## Potential Enhancements to the AVS (for learning purposes)
The architecture can be further enhanced via:
- the nature of the request is more sophisticated than generating a constant string
- the operators might need to coordinate with each other
- the type of signature is different based on the constraints of the service
- the type and amount of security used to secure the AVS


## Todos
- Rewrite the local devnet EigenLayer contract deployment steps to work without requiring the json config input file and remove deployment json output file.
- Reorganize Operator code folder. Migrate from typescript to simple javascript.
- Add operator demo steps for Holesky and Tenderly
- Add contract verification to the deploy scripts.
- Rebuild the hello world architecture in Excalidraw.