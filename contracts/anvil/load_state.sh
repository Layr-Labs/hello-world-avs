#!/bin/bash
set -e

STATE_FILE="contracts/anvil/state.json"

mkdir -p "$(dirname "$STATE_FILE")"

echo "Starting Anvil with state dump in background"
anvil --dump-state "$STATE_FILE" --host 0.0.0.0 --port 8545 --base-fee 0 --gas-price 0 &
ANVIL_PID=$!

echo "Deploying EigenLayer contracts."
make deploy-eigenlayer-contracts 

echo "Deploying HelloWorld contracts."
make deploy-helloworld-contracts

echo "Killed Anvil..."
kill $ANVIL_PID
