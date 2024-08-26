# Hello World AVS

Welcome to the Hello World AVS.

This project shows you the simplest functionality you can expect from an AVS.

It will give you a concrete understanding of the basic components.

![hello-world-png](./assets/hello-world-diagram.png)

There are 5 steps to this AVS:
- AVS consumer requests a "Hello World" message to be generated and signed
- AVS takes on the request by emitting an event for operators to pick up the request
- any operator who is staked to serve this AVS takes this request, generates this message and signs it
- the operator submits this message with their signature back to the AVS
- *if the operator is in fact registered to the AVS and has the minimum needed stake, the submission is accepted*

That's it. This simple flow highlights some of the core mechanics of how AVSs work.

Where additional sophistication with AVSs come into the picture:
- the nature of the request is more sophisticated than generating a constant string
- the operators might need to coordinate with each other
- the type of signature is different based on the constraints of the service
- the type and amount of security used to secure the AVS
- and so on...

## Quick Start

### Dependencies

1. [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
2. [Foundry](https://getfoundry.sh/)
3. [Docker](https://www.docker.com/get-started/)
   * Make sure Docker is running for automated deployment

Following global NodeJS packages:
1. [Typescript](https://github.com/microsoft/TypeScript)

## Typescript instructions

### Automated deployment (uses existing state file)

1. Run `npm install`
2. Run `cp .env.local .env`
3. Run `make start-chain-with-contracts-deployed`
    * This will build the contracts, start an Anvil chain, deploy the contracts to it, and leaves the chain running in the current terminal
4. Open new terminal tab and run `make start-operator`
    * This will compile the AVS software and start monitering new tasks
5. Open new terminal tab and run `make spam-tasks` (Optional)
    * This will spam the AVS with random names every 15 seconds

### Manual deployment

This walks you through how to manually deploy using Foundry (Anvil, Forge, and Cast)

1. Run `npm install` to install the TypeScript dependencies
2. Run `cp .env.local .env`
3. Compile the contracts.

```sh
cd contracts && forge build
```

4. Start Anvil by opening your terminal and running the following command:

```sh
anvil
```

5. In a separate terminal window, deploy the EigenLayer contracts.

```sh

cd contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts

forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url http://localhost:8545 \
--private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast \
--sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json
```

6. In a separate terminal window, deploy the AVS contracts.

```sh
cd contracts

forge script script/HelloWorldDeployer.s.sol --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast -v
```

7. Start the operator

```sh
tsc && node dist/index.js
```

8. In a separate window, start creating tasks

```sh
tsc && node dist/createNewTasks.js
```

## Rust instructions

### Automated deployment (uses existing state file)

1. Run `make start-chain-with-contracts-deployed`
    * This will build the contracts, start an Anvil chain, deploy the contracts to it, and leaves the chain running in the current terminal

2. Run `make start-rust-operator`

3. Run `make spam-rust-tasks`

Tests are supported in anvil only . Make sure to run the 1st command before running the  tests:

```
cargo test --workspace
```


##### Holesky Testnet

| Contract Name               | Holesky Address                                   |
| -------------               | -------------                                     |
| Hello World Service Manager | [0x3361953F4a9628672dCBcDb29e91735fb1985390](https://holesky.etherscan.io/address/0x3361953F4a9628672dCBcDb29e91735fb1985390)    |
| Delegation Manager          | [0xA44151489861Fe9e3055d95adC98FbD462B948e7](https://holesky.etherscan.io/address/0xA44151489861Fe9e3055d95adC98FbD462B948e7)                                           |
| Avs Directory               | [0x055733000064333CaDDbC92763c58BF0192fFeBf](https://holesky.etherscan.io/address/0x055733000064333CaDDbC92763c58BF0192fFeBf)      |

You don't need to run any script for holesky testnet.

1. Use the HOLESKY_ namespace env parameters in the code , instead of normal parameters.

2. Run `make start-rust-operator`

3. Run `make spam-rust-tasks `


## Extensions

- Operator needs a minimum stake amount to make submissions
- Add another strategy to the AVS
- Operator must respond within a certain number of blocks

## Deployment on Holesky

To deploy the Hello World AVS contracts to the Holesky network, follow these steps:

1. Ensure you have the necessary RPC URL and private key for the Holesky network.
2. Run the deployment script using Foundry:
    ```bash
    forge script script/HoleskyDeployer.s.sol:HoleskyDeployer --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -vvvv
    ```
    Replace `$RPC_URL` with your Holesky RPC URL and `$PRIVATE_KEY` with your private key.

## Adding a New Strategy

To add a new strategy to the Hello World AVS, follow the guide provided in [`AddNewStrategy.md`](https://github.com/Layr-Labs/hello-world-avs/blob/master/AddNewStrategy.md). This guide walks you through the necessary steps to add and whitelist a new strategy for the AVS.


## Tenderly Deployment

1) Follow the [Tenderly Virtual Testnet setup instructions](https://docs.tenderly.co/virtual-testnets/quickstart).

[Install the Tenderly CLI](https://github.com/Tenderly/tenderly-cli?tab=readme-ov-file#installation)

2) Configure Tenderly instance

```sh
# Login to your Tenderly account:
tenderly login

# Set env vars
TENDERLY_TESTNET_RPC_PUBLIC=https://virtual.holesky.rpc.tenderly.co/a58a7dde-1b39-47f6-8206-29f969f7e284
TENDERLY_TESTNET_RPC_ADMIN=https://virtual.holesky.rpc.tenderly.co/3f74d2f5-a603-43b1-9068-78d471de8b6c
PUBLIC_KEY=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# Fund account using the tenderly rpc
curl $TENDERLY_TESTNET_RPC_ADMIN \
-X POST \
-H "Content-Type: application/json" \
-d '{
    "jsonrpc": "2.0",
    "method": "tenderly_setBalance",
    "params": [
      [
        "${PUBLIC_KEY}"
        ],
      "0xDE0B6B3A7640000"
      ],
    "id": "1234"
}'

```



3) Deploy contracts using Foundry

```sh

TENDERLY_TESTNET_RPC_ADMIN=https://virtual.holesky.rpc.tenderly.co/a58a7dde-1b39-47f6-8206-29f969f7e284

forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url $TENDERLY_TESTNET_RPC_ADMIN \
--private-key $PRIVATE_KEY --broadcast -vvvv \
--sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json
```



7) Verify contracts?

forge verify-contract --verifier-url $TENDERLY_TESTNET_RPC_ADMIN 

Note: Ensure you have sufficient testnet ETH in your Tenderly account for deployments and transactions.