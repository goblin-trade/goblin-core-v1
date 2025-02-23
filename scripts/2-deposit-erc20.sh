#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly CONTRACT=0xa6e41ffd769491a42a6e5ce453259b93983a22ef
readonly NUM_CALLS=01
readonly DEPOSIT_ERC20_SELECTOR=01
readonly PAYLOAD_LEN=30 # 48 = 0x30
readonly TOKEN=7E32b54800705876d3b5cFbc7d9c226a211F7C1a
readonly RECIPIENT=3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E
readonly LOTS=0100000000000000

cast send "0x$TOKEN" \
    "approve(address,uint256)" $CONTRACT 10000000 \
    --rpc-url $RPC \
    --private-key $PRIVATE_KEY

cast send $CONTRACT \
    "0x$NUM_CALLS$DEPOSIT_ERC20_SELECTOR$PAYLOAD_LEN$TOKEN$RECIPIENT$LOTS" \
    --rpc-url $RPC \
    --private-key $PRIVATE_KEY

readonly GET_TRADER_STATE_SELECTOR=0A
readonly TRADER_KEY_LEN=28 # 48 = 0x28

# Check trader state
echo "Trader state-"
cast call $CONTRACT \
    "0x$NUM_CALLS$GET_TRADER_STATE_SELECTOR$TRADER_KEY_LEN$RECIPIENT$TOKEN" \
    --rpc-url $RPC
