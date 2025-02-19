#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly SPENDER=0x47cec0749bd110bc11f9577a70061202b1b6c034
readonly TOKEN=4Af567288e68caD4aA93A272fe6139Ca53859C70

cast send "0x$TOKEN" \
    "approve(address,uint256)" $SPENDER 21000000000000000000000000 \
    --rpc-url $RPC \
    --private-key $PRIVATE_KEY
