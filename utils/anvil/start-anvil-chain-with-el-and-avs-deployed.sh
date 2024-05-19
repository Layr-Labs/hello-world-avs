#!/bin/bash

RPC_URL=http://localhost:8545

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
echo "⚙️ Starting an anvil chain with the EigenLayer and Hello World AVS contracts deployed..."
# start an anvil instance in the background that has EigenLayer and Hello World AVS contracts deployed
start_anvil_docker $FULL_DEPLOYMENT_ANVIL_STATE_FILE ""

cd $CONTRACTS_DIR
# we need to restart the anvil chain at the correct block, otherwise the indexRegistry has a quorumUpdate at the block number
# at which it was deployed (aka quorum was created/updated), but when we start anvil by loading state file it starts at block number 0
# so calling getOperatorListAtBlockNumber reverts because it thinks there are no quorums registered at block 0
# advancing chain manually like this is a current hack until https://github.com/foundry-rs/foundry/issues/6679 is merged
cast rpc anvil_mine 100 --rpc-url $RPC_URL
space
echo "Advancing chain... current block-number:" $(cast block-number)

# Bring Anvil back to the foreground
docker logs anvil

space
echo "Attaching to Anvil chain container...Hello World AVS contracts are ready to be interacted with."
docker attach anvil
