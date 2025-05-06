#!/usr/bin/env bash

readonly NUM_CALLS=01
readonly WITHDRAW_ETH_SELECTOR=03

# 1 atom in big endian. 01 equals 1, each byte occupies 2 characters
readonly ATOMS=0100000000000000

cast send $CONTRACT \
    "0x$NUM_CALLS$WITHDRAW_ETH_SELECTOR${BASE_TOKEN#0x}${ADDRESS#0x}$ATOMS" \
    --private-key $PRIVATE_KEY

readonly GET_TRADER_STATE_SELECTOR=0A

# Check trader state
echo "Trader state-"
cast call $CONTRACT \
    "0x$NUM_CALLS$GET_TRADER_STATE_SELECTOR${ADDRESS#0x}${BASE_TOKEN#0x}"
