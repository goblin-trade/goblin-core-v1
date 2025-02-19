#!/bin/bash

readonly PRIVATE_KEY=0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
readonly RPC=http://127.0.0.1:8547

readonly TOKEN=0x4Af567288e68caD4aA93A272fe6139Ca53859C70
readonly ACCOUNT=0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E
readonly SPENDER=0xdb2d15a3eb70c347e0d2c2c7861cafb946baab48

# Call the balanceOf function on the ERC20 token contract
cast call $TOKEN "allowance(address,address)" $ACCOUNT $SPENDER --rpc-url $RPC
