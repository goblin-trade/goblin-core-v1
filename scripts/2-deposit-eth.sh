#!/usr/bin/env bash

readonly NUM_CALLS=01
readonly DEPOSIT_ETH_SELECTOR=00

# 10^18 atoms are normalized to 10^6 atoms. Therefore minimum size should be 10^12
# readonly VALUE=1000000
readonly VALUE=1000000000000

cast send $CONTRACT \
    "0x$NUM_CALLS$DEPOSIT_ETH_SELECTOR${ADDRESS#0x}" \
    --value $VALUE \
    --private-key $PRIVATE_KEY

readonly GET_TRADER_STATE_SELECTOR=0A
readonly NATIVE_TOKEN=0x0000000000000000000000000000000000000000

# Check trader state
echo "Trader state-"
cast call $CONTRACT \
    "0x$NUM_CALLS$GET_TRADER_STATE_SELECTOR${ADDRESS#0x}${NATIVE_TOKEN#0x}"
