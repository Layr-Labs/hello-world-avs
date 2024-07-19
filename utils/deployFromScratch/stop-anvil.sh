# Stop local anvil instance
ANVIL_PID=$(cat ./anvil.pid)

if [ -z "$ANVIL_PID" ]; then
    echo "Anvil PID does not exist. Anvil instance is not running."
else
    kill $ANVIL_PID
    rm -rf ./anvil.pid
fi