# Deploy AVS contracts

source .env.local

# Check if Anvil is running and the PID file exists
if ! pgrep -f anvil > /dev/null && [ -f ./anvil.pid ]; then
    echo "Anvil is not running or the PID file does not exist."
    exit 1
fi
# Continue with the deployment

# Change of directory is required in order for the forge script to work properly
cd ../../contracts
# Deploy AVS contracts. Note: the deployment process will require some time for compilation on the first run.
forge script script/HelloWorldDeployer.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast -v

# Note the file hello_world_avs_deployment_output.json contains the deployment data (addresses) of the deployed AVS contracts


# Send 10 ETH to the operator address to enable them to register with the eigenlayer contracts in future steps
cast send 0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5 --value 10ether --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6
