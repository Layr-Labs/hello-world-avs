#!/bin/bash

RPC_URL=http://localhost:8545
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
cd "$parent_path"

set -a
source ./utils.sh
set +a

trap 'cleanup $LINENO "$BASH_COMMAND"' EXIT

space
echo "⚙️ Starting an anvil chain in the background from state $EIGENLAYER_CONTRACTS_DEPLOYMENT_STATE_FILE and dumping its state to $FULL_DEPLOYMENT_ANVIL_STATE_FILE upon exit..."
# start an anvil instance in the background that has eigenlayer contracts deployed
start_anvil_docker $EIGENLAYER_CONTRACTS_DEPLOYMENT_STATE_FILE $FULL_DEPLOYMENT_ANVIL_STATE_FILE
echo "✅ Anvil chain started and state dumped successfully to $FULL_DEPLOYMENT_ANVIL_STATE_FILE"

cd $CONTRACTS_DIR
echo "🚀 Deploying HelloWorld contract..."
forge script script/HelloWorldDeployer.s.sol --legacy --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -v
# save the block-number in the genesis file which we also need to restart the anvil chain at the correct block
# otherwise the indexRegistry has a quorumUpdate at a high block number, and when we restart a clean anvil (without genesis.json) file
# it starts at block 0, and so calling getOperatorListAtBlockNumber reverts because it thinks there are no quorums registered at block 0
# EDIT: this doesn't actually work... since we can't both load a state and a genesis.json file... see https://github.com/foundry-rs/foundry/issues/6679
# will keep here in case this PR ever gets merged.
GENESIS_FILE=$ANVIL_DIR/genesis.json
TMP_GENESIS_FILE=$ANVIL_DIR/genesis.json.tmp
jq '.number = "'$(cast block-number)'"' $GENESIS_FILE >$TMP_GENESIS_FILE
mv $TMP_GENESIS_FILE $GENESIS_FILE

# we also do this here to make sure the operator has funds to register with the eigenlayer contracts
cast send 0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5 -j --value 10ether --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6

space
echo "✅ Hello World AVS contracts successfully deployed"
echo "💾 Deployment addresses are saved to $CONTRACTS_DIR/script/output/31337/hello_world_avs_deployment_output.json"
echo "💾 Anvil's latest state is saved to $FULL_DEPLOYMENT_ANVIL_STATE_FILE"
