#!/bin/bash

readonly NUM_CALLS=01
readonly DEPOSIT_ERC20_SELECTOR=01
readonly TOKEN=7E32b54800705876d3b5cFbc7d9c226a211F7C1a
readonly LOTS=0100000000000000

cast send $BASE_TOKEN \
    "approve(address,uint256)" $CONTRACT 10000000 \
    --private-key $PRIVATE_KEY

cast send $CONTRACT \
    "0x$NUM_CALLS$DEPOSIT_ERC20_SELECTOR${BASE_TOKEN#0x}${ADDRESS#0x}$LOTS" \
    --private-key $PRIVATE_KEY

readonly GET_TRADER_STATE_SELECTOR=0A

# Check trader state
echo "Trader state-"
cast call $CONTRACT \
    "0x$NUM_CALLS$GET_TRADER_STATE_SELECTOR${ADDRESS#0x}${BASE_TOKEN#0x}"
