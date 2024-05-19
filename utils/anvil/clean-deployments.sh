#!/bin/bash

set -e -o nounset

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

rm -rf $EIGENLAYER_CONTRACTS_DEPLOYMENT_STATE_FILE $FULL_DEPLOYMENT_ANVIL_STATE_FILE 
