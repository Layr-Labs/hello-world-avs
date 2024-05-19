#!/bin/bash

set -e -o nounset

# pinning at old foundry commit because of https://github.com/foundry-rs/foundry/issues/7502
FOUNDRY_IMAGE=ghcr.io/foundry-rs/foundry:nightly-5b7e4cb3c882b28f3c32ba580de27ce7381f415a

parent_path=$(
    cd "$(dirname "${BASH_SOURCE[0]}")"
    pwd -P
)

clean_up() {
    # Check if the exit status is non-zero
    exit_status=$?
    if [ $exit_status -ne 0 ]; then
        echo "Script exited due to set -e on line $1 with command '$2'. Exit status: $exit_status"
    fi
}
# Use trap to call the clean_up function when the script exits
trap 'clean_up $LINENO "$BASH_COMMAND"' ERR

space() {
    echo ""
    echo ""
}

# start_anvil_docker $LOAD_STATE_FILE $DUMP_STATE_FILE
start_anvil_docker() {
    LOAD_STATE_FILE=$1
    DUMP_STATE_FILE=$2
    LOAD_STATE_DOCKER_VOLUME_ARG=$([[ -z $LOAD_STATE_FILE ]] && echo "" || echo "-v $LOAD_STATE_FILE:/load-state.json")
    DUMP_STATE_DOCKER_VOLUME_ARG=$([[ -z $DUMP_STATE_FILE ]] && echo "" || echo "-v $DUMP_STATE_FILE:/dump-state.json")
    LOAD_STATE_ANVIL_ARG=$([[ -z $LOAD_STATE_FILE ]] && echo "" || echo "--load-state /load-state.json")
    DUMP_STATE_ANVIL_ARG=$([[ -z $DUMP_STATE_FILE ]] && echo "" || echo "--dump-state /dump-state.json")

    # trap 'docker stop anvil' EXIT

    echo ""
    echo "Starting an anvil chain in a docker container with the following arguments:"
    echo "- Loading state docker volume arg: $LOAD_STATE_DOCKER_VOLUME_ARG"
    echo "- Dumping state docker volume arg: $DUMP_STATE_DOCKER_VOLUME_ARG"
    echo "- Foundry image: $FOUNDRY_IMAGE"
    echo "- Load state anvil arg: $LOAD_STATE_ANVIL_ARG"
    echo "- Dump state anvil arg: $DUMP_STATE_ANVIL_ARG"
    echo ""

    docker run -d --rm --name anvil -p 8545:8545 $LOAD_STATE_DOCKER_VOLUME_ARG $DUMP_STATE_DOCKER_VOLUME_ARG \
        --entrypoint anvil \
        $FOUNDRY_IMAGE \
        $LOAD_STATE_ANVIL_ARG $DUMP_STATE_ANVIL_ARG --host 0.0.0.0 -s 1
}
