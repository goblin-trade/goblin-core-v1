#!/bin/bash

# Transaction hash to trace
TX_HASH="0x2d00227e53ffb6b7fbb165b1b5c1945c6a18db9dd4a669edc4fba6ac959ec9e6"

# JSON-RPC request payload
JSON_PAYLOAD=$(cat <<EOF
{
  "jsonrpc": "2.0",
  "method": "debug_traceTransaction",
  "params": [
    "$TX_HASH",
    {"tracer": "stylusTracer"}
  ],
  "id": 1
}
EOF
)

# Make the curl request
curl -X POST \
  -H "Content-Type: application/json" \
  --data "$JSON_PAYLOAD" \
  $ETH_RPC_URL
