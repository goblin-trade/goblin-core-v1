#!/usr/bin/env bash

readonly NUM_CALLS=01
readonly DEPOSIT_ERC20_SELECTOR=01

# 1 atom (big endian)
readonly ATOMS=0100000000000000

# Approve 10^18 raw atoms
cast send $BASE_TOKEN \
    "approve(address,uint256)" $CONTRACT 1000000000000000000 \
    --private-key $PRIVATE_KEY

cast send $CONTRACT \
    "0x$NUM_CALLS$DEPOSIT_ERC20_SELECTOR${BASE_TOKEN#0x}${ADDRESS#0x}$ATOMS" \
    --private-key $PRIVATE_KEY

readonly GET_TRADER_STATE_SELECTOR=0A

# Check trader state
echo "Trader state-"
cast call $CONTRACT \
    "0x$NUM_CALLS$GET_TRADER_STATE_SELECTOR${ADDRESS#0x}${BASE_TOKEN#0x}"
