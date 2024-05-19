#!/bin/bash

set -e -o nounset

root_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)

# pinning at old foundry commit because of https://github.com/foundry-rs/foundry/issues/7502
FOUNDRY_IMAGE=ghcr.io/foundry-rs/foundry:nightly-5b7e4cb3c882b28f3c32ba580de27ce7381f415a
LOG=${LOG:-INFO}

# Directories 
TOP_LEVEL_DIR="$(dirname $(dirname $root_path))"
ANVIL_DIR="$TOP_LEVEL_DIR/utils/anvil"
CONTRACTS_DIR="$TOP_LEVEL_DIR/contracts"
EIGENLAYER_MIDDLEWARE_CONTRACTS_DIR="$CONTRACTS_DIR/lib/eigenlayer-middleware"
EIGENLAYER_CORE_CONTRACTS_DIR="$EIGENLAYER_MIDDLEWARE_CONTRACTS_DIR/lib/eigenlayer-contracts"

cleanup() {
    echo "Executing cleanup function..."
    set +e
    docker rm -f anvil
    # Give the docker container some time to shut down
    sleep 2
    exit_status=$?
    if [ $exit_status -ne 0 ]; then
        echo "Script exited due to set -e on line $1 with command '$2'. Exit status: $exit_status"
    fi
}

# Use trap to call the clean_up function when the script exits
trap 'cleanup $LINENO "$BASH_COMMAND"' ERR

space() {
    echo ""
    echo ""
}

# STATE LOCATIONS
# They don't end in .json because anvil --dump state will treat these as directories and write the state.json to a file inside them
EIGENLAYER_CONTRACTS_DEPLOYMENT_STATE_FILE="$ANVIL_DIR/eigenlayer-deployed-anvil-state.json"
FULL_DEPLOYMENT_ANVIL_STATE_FILE="$ANVIL_DIR/hello-world-and-eigenlayer-contracts-deployed-anvil-state.json"

# start_anvil_docker $LOAD_STATE_FILE $DUMP_STATE_FILE
start_anvil_docker() {
    LOAD_STATE_FILE=$1
    DUMP_STATE_FILE=$2
    LOAD_STATE_DOCKER_VOLUME_ARG=$([[ -z $LOAD_STATE_FILE ]] && echo "" || echo "-v $LOAD_STATE_FILE:/load-state.json")
    DUMP_STATE_DOCKER_VOLUME_ARG=$([[ -z $DUMP_STATE_FILE ]] && echo "" || echo "-v $DUMP_STATE_FILE:/dump-state.json")
    LOAD_STATE_ANVIL_ARG=$([[ -z $LOAD_STATE_FILE ]] && echo "" || echo "--load-state /load-state.json")
    DUMP_STATE_ANVIL_ARG=$([[ -z $DUMP_STATE_FILE ]] && echo "" || echo "--dump-state /dump-state.json")

    echo "dump state file: $DUMP_STATE_FILE"
    if [ ! -e "$DUMP_STATE_FILE" ] && [ "$DUMP_STATE_FILE" != "" ]; then
        echo "Creating empty file $DUMP_STATE_FILE"
        touch "$DUMP_STATE_FILE"
    fi

    if [ "$LOG" == "DEBUG" ]; then
        echo ""
        echo "Starting an anvil chain in a docker container with the following arguments:"
        echo "- Loading state docker volume arg: $LOAD_STATE_DOCKER_VOLUME_ARG"
        echo "- Dumping state docker volume arg: $DUMP_STATE_DOCKER_VOLUME_ARG"
        echo "- Foundry image: $FOUNDRY_IMAGE"
        echo "- Load state anvil arg: $LOAD_STATE_ANVIL_ARG"
        echo "- Dump state anvil arg: $DUMP_STATE_ANVIL_ARG"
        echo ""
    fi

    docker run -d --rm --name anvil -p 8545:8545 $LOAD_STATE_DOCKER_VOLUME_ARG $DUMP_STATE_DOCKER_VOLUME_ARG \
        --entrypoint anvil \
        $FOUNDRY_IMAGE \
        $LOAD_STATE_ANVIL_ARG $DUMP_STATE_ANVIL_ARG --host 0.0.0.0 -s 1
}
