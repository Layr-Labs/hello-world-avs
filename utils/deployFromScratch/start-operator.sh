source .env.local

# Send 10 ETH to the operator address to enable them to register with the eigenlayer contracts in the next step
cast send 0x860B6912C2d0337ef05bbC89b0C2CB6CbAEAB4A5 --value 10ether --private-key 0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6

cast send ${PRIVATE_KEY} --value 10ether

# Start the operator process if not already running
if [ -f operator.pid ]; then
    echo "Operator process is already running with PID $(cat operator.pid)"
else
    if (cd ../../ tsc && node dist/index.js)  > operator.log 2>&1 & then
        echo $! > operator.pid
    fi
fi