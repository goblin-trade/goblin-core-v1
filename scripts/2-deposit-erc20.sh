#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly CONTRACT=0xe1080224b632a93951a7cfa33eeea9fd81558b5e
readonly TOKEN=A6E41fFD769491a42A6e5Ce453259b93983a22EF
readonly SENDER=3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E
readonly RECIPIENT=$SENDER
readonly LOTS=1000000000000000

# cast send "0x$TOKEN" \
#     "approve(address,uint256)" $CONTRACT 21000000000000000000000000 \
#     --rpc-url $RPC \
#     --private-key $PRIVATE_KEY

cast send $CONTRACT \
    "0x01$TOKEN$SENDER$RECIPIENT$LOTS" \
    --rpc-url $RPC \
    --private-key $PRIVATE_KEY
