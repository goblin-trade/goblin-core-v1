#!/usr/bin/env bash

readonly NUM_CALLS=01
readonly WITHDRAW_ETH_SELECTOR=02
# 1 atom in big endian. 01 equals 1, each byte occupies 2 characters
readonly ATOMS=0100000000000000

cast send $CONTRACT \
    "0x$NUM_CALLS$WITHDRAW_ETH_SELECTOR${ADDRESS#0x}$ATOMS" \
    --private-key $PRIVATE_KEY

readonly GET_TRADER_STATE_SELECTOR=0A
readonly NATIVE_TOKEN=0x0000000000000000000000000000000000000000

# Check trader state
echo "Trader state-"
cast call $CONTRACT \
    "0x$NUM_CALLS$GET_TRADER_STATE_SELECTOR${ADDRESS#0x}${NATIVE_TOKEN#0x}"
