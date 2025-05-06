#!/usr/bin/env bash

# ./0-setup-nitro.sh

# Deploy create3 factory at nonce 3
cd goblin-localnet-scripts
forge script script/DeployCREATE3Factory.s.sol:DeployCREATE3Factory \
    --private-key $PRIVATE_KEY \
    --rpc-url $ETH_RPC_URL \
    --broadcast \
    --skip-simulation

# Base token at nonce 4
readonly BASE_TOKENS_TO_MINT=100000000000000000000
forge create \
    --private-key $PRIVATE_KEY --broadcast \
    TestERC20 --constructor-args "Base" "BASE" $BASE_TOKENS_TO_MINT

# Quote token at nonce 5
readonly QUOTE_TOKENS_TO_MINT=100000000000000000000
forge create \
    --private-key $PRIVATE_KEY --broadcast \
    TestERC20 --constructor-args "Quote" "QUOTE" $QUOTE_TOKENS_TO_MINT

cd ..

# Deploy goblin core
cargo build --release --target wasm32-unknown-unknown
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core_v1.wasm --endpoint $ETH_RPC_URL

# Compile init code
cargo run --example compile-contract

# Deploy goblin_core_v1 with CREATE3
readonly INIT_CODE=0x$(xxd -p target/wasm32-unknown-unknown/release/goblin_core_v1.contract | tr -d '\n')

cast send $CREATE3_FACTORY \
    "deploy(bytes32,bytes)" $GOBLIN_SALT $INIT_CODE \
    --private-key $PRIVATE_KEY

# Activate contract
cast send $ARB_WASM_CONTRACT \
    "activateProgram(address)" $CONTRACT \
    --private-key $PRIVATE_KEY \
    --value 0.0001ether
