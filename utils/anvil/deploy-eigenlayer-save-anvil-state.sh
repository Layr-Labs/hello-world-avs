#!/bin/bash

root_path=$(
    cd "$(dirname $(dirname $(dirname "${BASH_SOURCE[0]}")))"
    pwd -P
)

# cd to the directory of this script so that this can be run from anywhere
parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)
# At this point we are in tests/anvil
cd "$parent_path"

set -a
source ./utils.sh
set +a

# Directories 
TOP_LEVEL_DIR="$root_path"
ANVIL_DIR="$root_path/utils/anvil"
TOP_LEVEL_CONTRACTS_DIR="$root_path/contracts"
EIGENLAYER_MIDDLEWARE_CONTRACTS_DIR="$TOP_LEVEL_CONTRACTS_DIR/lib/eigenlayer-middleware"
EIGENLAYER_CORE_CONTRACTS_DIR="$EIGENLAYER_MIDDLEWARE_CONTRACTS_DIR/lib/eigenlayer-contracts"

# State files
DUMP_STATE_FILE="$ANVIL_DIR/eigenlayer-deployed-anvil-state.json"

cleanup() {
    echo "Executing cleanup function..."
    set +e
    docker rm -f anvil
    exit_status=$?
    if [ $exit_status -ne 0 ]; then
        echo "Script exited due to set -e on line $1 with command '$2'. Exit status: $exit_status"
    fi
}
trap 'cleanup $LINENO "$BASH_COMMAND"' EXIT

echo "⚙️ Starting an anvil chain in the background and dumping its state to $DUMP_STATE_FILE upon exit..."
if [ ! -e "$DUMP_STATE_FILE" ]; then
    echo "Creating empty file $DUMP_STATE_FILE"
    touch "$DUMP_STATE_FILE"
fi

start_anvil_docker "" $DUMP_STATE_FILE
if [ $? -ne 0 ]; then
    echo "❌ Failed to start an empty anvil chain in the background and dump its state to a json file upon exit"
    exit 1
fi
echo "✅ Anvil chain started and state dumped successfully to $DUMP_STATE_FILE"
space


cd $EIGENLAYER_CORE_CONTRACTS_DIR
echo "Changing directory to $EIGENLAYER_CORE_CONTRACTS_DIR"
EXISTING_DEPLOYMENT_DATA_FROM_SCRATCH=script/output/devnet/M2_from_scratch_deployment_data.json
if [ -e "$EXISTING_DEPLOYMENT_DATA_FROM_SCRATCH" ]; then
    echo "Deployment overwrites the previous deployment file so we save it as backup, because we want the new deployment output in our local files, and not in the eigenlayer-contracts submodule files"
    mv script/output/devnet/M2_from_scratch_deployment_data.json script/output/devnet/M2_from_scratch_deployment_data.json.bak
fi

# M2_Deploy_From_Scratch.s.sol prepends "script/testing/" to the configFile passed as input (M2_deploy_from_scratch.anvil.config.json)
space
echo "🚀 Deploying EigenLayer core contracts from scratch..."
forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --legacy --rpc-url http://localhost:8545 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json
if [ $? -ne 0 ]; then
    echo "❌ Failed to deploy EigenLayer core contracts from scratch"
    exit 1
fi

LOCAL_DEPLOYMENT_OUTPUT_FILE=$TOP_LEVEL_CONTRACTS_DIR/script/output/31337/eigenlayer_deployment_output.json
mv script/output/devnet/M2_from_scratch_deployment_data.json $LOCAL_DEPLOYMENT_OUTPUT_FILE
space
echo "✅ Clean-state EigenLayer core contracts successfully deployed"
echo "💾 Deployment addresses are saved to $LOCAL_DEPLOYMENT_OUTPUT_FILE"
echo "💾 Anvil's latest state is saved to $DUMP_STATE_FILE"
sleep 2
mv script/output/devnet/M2_from_scratch_deployment_data.json.bak script/output/devnet/M2_from_scratch_deployment_data.json
space
