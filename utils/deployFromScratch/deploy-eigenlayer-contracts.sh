## Deploy EigenLayer contracts

source .env.local

# Check if Anvil is running and the PID file exists
if ! pgrep -f anvil > /dev/null && [ -f ./anvil.pid ]; then
    echo "Anvil is not running or the PID file does not exist."
    exit 1
fi
# Continue with the deployment

# Change of directory is required in order for the forge script to work properly
cd ../../contracts/lib/eigenlayer-middleware/lib/eigenlayer-contracts
# Deploy EigenLayer contracts. Note: the deployment process will require some time for compilation on the first run.
forge script script/deploy/devnet/M2_Deploy_From_Scratch.s.sol --rpc-url $RPC_URL --private-key $PRIVATE_KEY --broadcast --sig "run(string memory configFile)" -- M2_deploy_from_scratch.anvil.config.json

# Note the file M2_from_scratch_deployment_data.json contains the deployment data (addresses) of the deployed EigenLayer contracts

