#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly TOKEN=0xA6E41fFD769491a42A6e5Ce453259b93983a22EF
readonly ACCOUNT=0x84401cd7abbebb22acb7af2becfd9be56c30bcf1

# Call the balanceOf function on the ERC20 token contract
cast call $TOKEN "balanceOf(address)(uint256)" $ACCOUNT --rpc-url $RPC
