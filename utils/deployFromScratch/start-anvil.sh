## Load .env variables
source .env.local

# Start the anvil chain locally
if [ -f anvil.pid ]; then
    echo "Anvil process is already running with PID $(cat anvil.pid)"
else
    anvil > anvil.log 2>&1 &
    echo $! > anvil.pid
fi





