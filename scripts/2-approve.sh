#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly SPENDER=0xfb8c3906979fa82ed9e9e18c3ee21995761a13e7
readonly TOKEN=A6E41fFD769491a42A6e5Ce453259b93983a22EF

cast send "0x$TOKEN" \
    "approve(address,uint256)" $SPENDER 21000000000000000000000000 \
    --rpc-url $RPC \
    --private-key $PRIVATE_KEY
