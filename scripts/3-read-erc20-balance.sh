#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly TOKEN=0x7E32b54800705876d3b5cFbc7d9c226a211F7C1a
readonly ACCOUNT=0xa6e41ffd769491a42a6e5ce453259b93983a22ef

# Call the balanceOf function on the ERC20 token contract
cast call $TOKEN "balanceOf(address)(uint256)" $ACCOUNT --rpc-url $RPC
