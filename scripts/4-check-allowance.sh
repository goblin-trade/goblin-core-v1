#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly TOKEN=0xA6E41fFD769491a42A6e5Ce453259b93983a22EF
readonly ACCOUNT=0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E
readonly SPENDER=0x33fa6ca777cc0ca84d39e5f4fef5e0e1bd87a68d

# Call the balanceOf function on the ERC20 token contract
cast call $TOKEN "allowance(address,address)" $ACCOUNT $SPENDER --rpc-url $RPC
