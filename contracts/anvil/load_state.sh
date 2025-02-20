#!/bin/bash
set -e

STATE_FILE="contracts/anvil/state.json"

mkdir -p "$(dirname "$STATE_FILE")"

echo "Starting Anvil with state dump in background"
anvil --dump-state "$STATE_FILE" &
ANVIL_PID=$!

sleep 3

echo "Deploying EigenLayer contracts."
make deploy-eigenlayer-contracts 

echo "Deploying HelloWorld contracts."
make deploy-helloworld-contracts

echo "Killed Anvil..."
kill $ANVIL_PID
