# Hello World AVS

Welcome to the Hello World AVS.

This project shows you the simplest functionality you can expect from an AVS.

It will give you a concrete understanding of the basic components.

![hello-world-png](./assets/hello-world-diagram.png)

There are only 3 main steps to this AVS:
- accepts a request to generate a "Hello World" message
- any operator who is serving this AVS
    - takes this request and generates this message
    - signs this message
    - submits this message with their signature back to the AVS
- *if the operator is in fact registered to the AVS, the submission is accepted*

That's it. This simple flow highlights some of the core mechanics of how AVSs work.

Where additional sophistication with AVSs come into the picture:
- the nature of the request is more sophisicated than generating a constant string
- the operators might need to coordinate with each other
- the type of signature is different based on the constraints of the service
- the type and amount of security used to secure the AVS
- and so on...

## Quick Start

### Dependencies 

1. [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
2. [Foundry](https://getfoundry.sh/)
3. [Docker](https://www.docker.com/get-started/)

### Steps

1. Make sure Docker is running
2. Run `start-chain-and-deploy-hello-world-avs`
    2.1 This will build the contracts, deploy them, and leaves an Anvil chain running in the current terminal

## Extensions

- Operator needs a minimum stake amount to make submissions
- Add another strategy to the AVS
- Operator must respond within a certain number of blocks