#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly SPENDER=0xacca32fccaf3220c1a3a31f7a5f879c231320642
readonly TOKEN=A6E41fFD769491a42A6e5Ce453259b93983a22EF

cast send "0x$TOKEN" \
    "approve(address,uint256)" $SPENDER 21000000000000000000000000 \
    --rpc-url $RPC \
    --private-key $PRIVATE_KEY
