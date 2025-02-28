#!/bin/bash

echo "balance"
cast call $BASE_TOKEN "balanceOf(address)(uint256)" $ADDRESS

echo "allowance"
cast call $BASE_TOKEN "allowance(address,address)" $ADDRESS $CONTRACT

# Check trader state
readonly NUM_CALLS=01
readonly GET_TRADER_STATE_SELECTOR=0A

echo "Trader state"
cast call $CONTRACT \
    "0x$NUM_CALLS$GET_TRADER_STATE_SELECTOR${ADDRESS#0x}${BASE_TOKEN#0x}"
