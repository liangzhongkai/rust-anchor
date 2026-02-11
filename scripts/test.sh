#!/usr/bin/env bash
set -e
cd "$(dirname "$0")/.."

# Anchor 0.32's default Surfpool often fails to deploy. Use solana-test-validator instead.
# Kill any existing validator (Surfpool or solana-test-validator)
pkill -f solana-test-validator 2>/dev/null || true
pkill -f surfpool 2>/dev/null || true
sleep 2

validator_ready() {
  curl -s -X POST http://localhost:8899 -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}' 2>/dev/null | grep -q '"result":"ok"'
}

echo "Starting solana-test-validator..."
solana-test-validator --reset > /tmp/validator.log 2>&1 &
VALIDATOR_PID=$!

echo "Waiting for validator..."
for i in {1..30}; do
  if validator_ready; then
    echo "Validator ready."
    break
  fi
  if [ $i -eq 30 ]; then
    echo "Validator failed to start. Check /tmp/validator.log"
    kill $VALIDATOR_PID 2>/dev/null || true
    exit 1
  fi
  sleep 1
done

echo "Deploying program..."
anchor deploy --provider.cluster localnet

# Run TypeScript tests
echo "Running tests..."
ANCHOR_PROVIDER_URL=http://localhost:8899 ANCHOR_WALLET=$HOME/.config/solana/id.json \
  yarn run ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'
EXIT_CODE=$?

# Cleanup if we started the validator
[ -n "$VALIDATOR_PID" ] && kill $VALIDATOR_PID 2>/dev/null || true
exit $EXIT_CODE
