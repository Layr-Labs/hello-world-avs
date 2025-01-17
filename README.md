# Hello World AVS

Welcome to the Hello World AVS. This project shows you the simplest functionality you can expect from an AVS. It will give you a concrete understanding of the basic components. For new users, please find [this video walkthrough](https://drive.google.com/file/d/1P6uA6kYWCbpeorTjADuoTlQ-q8uqwPZf/view?usp=sharing) of the hello world AVS repository.

## Architecture

![hello-world-png](./assets/hello-world-diagramv2.png)

### AVS User Flow

1) AVS consumer requests a "Hello World" message to be generated and signed.
2) HelloWorld contract receives the request and emits a NewTaskCreated event for the request.
3) All Operators who are registered to the AVS and has staked, delegated assets takes this request. Operator generates the requested message, hashes it, and signs the hash with their private key.
4) Each Operator submits their signed hash back to the HelloWorld AVS contract.
5) If the Operator is registered to the AVS and has the minimum needed stake, the submission is accepted.

That's it. This simple flow highlights some of the core mechanics of how AVSs work.

# Local Devnet Deployment

The following instructions explain how to manually deploy the AVS from scratch including EigenLayer and AVS specific contracts using Foundry (forge) to a local anvil chain, and start Typescript Operator application and tasks.

## Development Environment
This section describes the tooling required for local development.

### Non-Nix Environment
Install dependencies:

- [Node](https://nodejs.org/en/download/)
- [Typescript](https://www.typescriptlang.org/download)
- [ts-node](https://www.npmjs.com/package/ts-node)
- [tcs](https://www.npmjs.com/package/tcs#installation)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [Foundry](https://getfoundry.sh/)
- [ethers](https://www.npmjs.com/package/ethers)

### Nix Environment 
On [Nix](https://nixos.org/) platforms, if you already have the proper Nix configuration, you can build the project’s artifacts inside a `nix develop` shell
``` sh
nix develop
```
Otherwise, please refer to [installed and configured](./docs/nix-setup-guide.md) section.

## Quick start

### Start Anvil Chain

In terminal window #1, execute the following commands:

```sh

# Install npm packages
npm install

# Start local anvil chain
npm run start:anvil
```

### Deploy Contracts and Start Operator

Open a separate terminal window #2, execute the following commands

```sh
# Setup .env file
cp .env.example .env
cp contracts/.env.example contracts/.env

# Updates dependencies if necessary and builds the contracts 
npm run build

# Deploy the EigenLayer contracts
npm run deploy:core

# Deploy the Hello World AVS contracts
npm run deploy:hello-world

# (Optional) Update ABIs
npm run extract:abis

# Start the Operator application
npm run start:operator

```

### Create and Claim Payments

In a terminal, start a new instance of anvil and deploy the core and avs contracts
```sh
# Start anvil
npm run start:anvil-quick
# Deploy the EigenLayer contracts
npm run deploy:core

# Deploy the Hello World AVS contracts
npm run deploy:hello-world

```

In another terminal, run:

```sh
# Create payment roots
npm run create-payments-root

# Claim created payment
npm run claim-payments
```

To run operator directed payments, run:
```sh
#Create payment roots
npm run create-operator-directed-payments-root

# Claim created payment
npm run claim-payments
```

In order to create and claim multiple payments (run the above two commands more than once), you must wait up to 5 minutes.




### Create Hello-World-AVS Tasks

Open a separate terminal window #3, execute the following commands

```sh
# Start the createNewTasks application 
npm run start:traffic
```

### Help and Support

For help and support deploying and modifying this repo for your AVS, please:

1. Open a ticket via the intercom link at [support.eigenlayer.xyz](https://support.eigenlayer.xyz).
2. Include the necessary troubleshooting information for your environment:
  * Local anvil testing:
    * Redeploy your local test using `--revert-strings debug` flag via the following commands and retest: `npm run deploy:core-debug && npm run deploy:hello-world-debug`
    * Include the full stacktrace from your error as a .txt file attachment.
    * Create a minimal repo that demonstrates the behavior (fork or otherwise)
    * Steps require to reproduce issue (compile and cause the error)
  * Holesky testing:
    * Ensure contracts are verified on Holesky. Eg `forge verify-contract --chain-id 17000 --num-of-optimizations 200 src/YourContract.sol:YourContract YOUR_CONTRACT_ADDRESS`
    * Send us your transaction hash where your contract is failing. We will use Tenderly to debug (adjust gas limit) and/or cast to re-run the transaction (eg `cast call --trace "trace_replayTransaction(0xTransactionHash)"`).


### Contact Us

If you're planning to build an AVS and would like to speak with a member of the EigenLayer DevRel team to discuss your ideas or architecture, please fill out this form and we'll be in touch shortly: [EigenLayer AVS Intro Call](https://share.hsforms.com/1BksFoaPjSk2l3pQ5J4EVCAein6l)


### Disclaimers

- This repo is meant currently intended for _local anvil development testing_. Holesky deployment support will be added shortly.
- Users who wish to build an AVS for Production purposes will want to migrate from the `ECDSAServiceManagerBase` implementation in `HelloWorldServiceManager.sol` to a BLS style architecture using [RegistryCoordinator](https://github.com/Layr-Labs/eigenlayer-middleware/blob/dev/docs/RegistryCoordinator.md).

# Appendix (Future Capabilities In Progress)

## Adding a New Strategy

## Potential Enhancements to the AVS (for learning purposes)

The architecture can be further enhanced via:

- the nature of the request is more sophisticated than generating a constant string
- the operators might need to coordinate with each other
- the type of signature is different based on the constraints of the service
- the type and amount of security used to secure the AVS

## Rust Operator instructions

### Anvil Deployment

1. Start Anvil Chain

In terminal window #1, execute the following commands:
```sh
anvil
```

2. Deploy Contracts

Open a separate terminal window #2, execute the following commands

```
make deploy-eigenlayer-contracts

make deploy-helloworld-contracts
```

3. Start Operator

```sh
make start-rust-operator
```
4. Spam Tasks

```sh
make spam-rust-tasks
```

### Testing

1. Start Anvil Chain

In terminal window #1, execute the following commands:
```sh
anvil
```

2. Deploy Contracts

Open a separate terminal window #2, execute the following commands

```
make deploy-eigenlayer-contracts

make deploy-helloworld-contracts
```

3. Run this command

```
cargo test --workspace
```