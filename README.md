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
- _if the operator is in fact registered to the AVS and has the minimum needed stake, the submission is accepted_

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
   - Make sure Docker is running
4. jq - Can be installed using sudo apt-get install jq

Following NodeJS packages:

1. tcs
2. ethers

### Steps

#### Typescript

1. Run `yarn install`
2. Run `cp .env.local .env`
3. Run `make start-chain-with-contracts-deployed`
   - This will build the contracts, start an Anvil chain, deploy the contracts to it, and leaves the chain running in the current terminal
   - Once the chain is running, open the container's logs and choose a private key from the list of private keys available. This will be used to run the operator as these private keys come preloaded with ETH.
4. Open new terminal tab and run `make start-operator`
   - This will compile the AVS software and start monitoring new tasks
   - Note: If you stop the operator and want to restart it, but your wallet is already registered as an operator, you have to first go to '/operator/index.ts' and comment this line: `await registerOperator();` as that operator would fail.
5. Open new terminal tab and run `make spam-tasks` (Optional)
   - This will spam the AVS with random names every 15 seconds

#### Rust lang

##### Anvil

1. Run `make start-chain-with-contracts-deployed`

   - This will build the contracts, start an Anvil chain, deploy the contracts to it, and leaves the chain running in the current terminal

2. Run `make start-rust-operator`

   - Similar to the other operator, if the key is already registered as an operator, one has to comment these lines out from 'operator/rust/crates/operator/src/start_operator.rs':

   ```
   if let Err(e) = register_operator().await {
        eprintln!("Failed to register operator: {:?}", e);
        return;
   }
   ```

3. Run `make spam-rust-tasks`

Tests are supported in anvil only . Make sure to run the 1st command before running the tests:

```

cargo test --workspace

```

##### Holesky Testnet

| Contract Name               | Holesky Address                                                                                                               |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Hello World Service Manager | [0x3361953F4a9628672dCBcDb29e91735fb1985390](https://holesky.etherscan.io/address/0x3361953F4a9628672dCBcDb29e91735fb1985390) |
| Delegation Manager          | [0xA44151489861Fe9e3055d95adC98FbD462B948e7](https://holesky.etherscan.io/address/0xA44151489861Fe9e3055d95adC98FbD462B948e7) |
| Avs Directory               | [0x055733000064333CaDDbC92763c58BF0192fFeBf](https://holesky.etherscan.io/address/0x055733000064333CaDDbC92763c58BF0192fFeBf) |

You don't need to run any script for holesky testnet.

1. Use the HOLESKY\_ namespace env parameters in the code , instead of normal parameters.

2. Run `make start-rust-operator`

3. Run `make spam-rust-tasks `

## Extensions

- Operator needs a minimum stake amount to make submissions
- Add another strategy to the AVS
- Operator must respond within a certain number of blocks

## Deployment on local Anvil chain

- Run `make deploy-hello-world-contracts-to-anvil-and-save-state`
- This will build and deploy your modified version of the contract to the Anvil chain and save its state so that whenever you try running `make start-chain-with-contracts-deployed` again, the chain can start with the new contracts

## Deployment on Holesky

To deploy the Hello World AVS contracts to the Holesky network, follow these steps:

1. Ensure you have the necessary RPC URL and private key for the Holesky network.
2. Change the directory that is opened in the terminal to 'contracts'
3. Run the deployment script using Foundry:
   ```bash
   forge script script/HoleskyDeployer.s.sol:HoleskyDeployer --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -vvvv
   ```
   Replace `$RPC_URL` with your Holesky RPC URL and `$PRIVATE_KEY` with your private key.
4. Check out the 'script/output/' folder where you will find a list of the deployed contracts on chain
   - Note: You can put those addresses inside the .env file to use them

## Adding a New Strategy

To add a new strategy to the Hello World AVS, follow the guide provided in [`AddNewStrategy.md`](https://github.com/Layr-Labs/hello-world-avs/blob/master/AddNewStrategy.md). This guide walks you through the necessary steps to add and whitelist a new strategy for the AVS.
