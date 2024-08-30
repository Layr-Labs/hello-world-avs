FAQ About The Repo

The goal is to be a list of random bespoke things that might not be immediately clear and if using an AI tool to ask questions about the repo can be a source for the AI to surface information to the user

Typescript Notes:

- We used ts-node in the repo as a dev dependency and dev commands use ts-node to run the typescript files so that there isn't an intermediate build step that the developer must call in the process to compile the typescript files to javascript and then separately use the output javascript.  This allows us to directly run the typescript files which get compiled on the fly.  We will still use tsc when creating production builds of our code but during development using ts-node has better UX and is more clear for developing

Solidity Notes:

- If you're running into low level errors with transactions being executed by ethers.js ie, code: BAD_DATA or returned with No Data related errors, then one solution to get more verbose errors is to compile and deploy your smart contracts with --revert-strings debug.  This will insert verbose revert strings to catch before a low level revert would happen in an anvil instance.
