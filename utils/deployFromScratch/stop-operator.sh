# Stop local operator instance
OPERATOR_PID=$(cat ./operator.pid)

if [ -z "$OPERATOR_PID" ]; then
    echo "operator PID does not exist. operator instance is not running."
else
    kill $OPERATOR_PID
    rm -rf ./operator.pid
fi