# TODO remove, this is deployed by default
#!/usr/bin/env bash
# Set $ADDRESS as admin with initial balance
# Problem- this changes the nonces

# Make the caller a chain owner
echo "Setting chain owner to pre-funded dev account..."
cast send 0x00000000000000000000000000000000000000FF "becomeChainOwner()" \
  --private-key $PRIVATE_KEY \
  --rpc-url $ETH_RPC_URL

# Deploy Cache Manager Contract
echo "Deploying Cache Manager contract..."
deploy_output=$(cast send --private-key $PRIVATE_KEY \
  --rpc-url $ETH_RPC_URL \
  --create 0x60a06040523060805234801561001457600080fd5b50608051611d1c61003060003960006105260152611d1c6000f3fe)

# Extract contract address using awk from plain text output
contract_address=$(echo "$deploy_output" | awk '/contractAddress/ {print $2}')

# Check if contract deployment was successful
if [[ -z "$contract_address" ]]; then
  echo "Error: Failed to extract contract address. Full output:"
  echo "$deploy_output"
  exit 1
fi

echo "Cache Manager contract deployed at address: $contract_address"

# Register the deployed Cache Manager contract
echo "Registering Cache Manager contract as a WASM cache manager..."
registration_output=$(cast send --private-key $PRIVATE_KEY \
  --rpc-url $ETH_RPC_URL \
  0x0000000000000000000000000000000000000070 \
  "addWasmCacheManager(address)" "$contract_address")
