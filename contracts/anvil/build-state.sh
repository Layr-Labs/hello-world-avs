#!/usr/bin/env bash

set -e

STATE_FILE="contracts/anvil/state.json"

mkdir -p "$(dirname "$STATE_FILE")"

echo "Starting Anvil with state dump in background"
anvil --dump-state "$STATE_FILE" --port 8545 --base-fee 0 --gas-price 0 > /dev/null 2>&1 &
ANVIL_PID=$!

sleep 3

cp .env.example .env
cp contracts/.env.example contracts/.env

echo "Building contracts"
make build-contracts > /dev/null 2>&1

echo "Deploying EigenLayer contracts."
make deploy-eigenlayer-contracts > /dev/null 2>&1

echo "Deploying HelloWorld contracts."
make deploy-helloworld-contracts > /dev/null 2>&1

echo "Killed Anvil"
kill $ANVIL_PID || true
