#!/usr/bin/env bash

# To use nitro-devnode instead
cd ./nitro-devnode
bash ./run-dev-node.sh

# rm -rf /tmp/dev-test/
# ./nitro/target/bin/nitro --dev \
#     --chain.dev-wallet.private-key ${PRIVATE_KEY#0x} \
#     --execution.vmtrace.tracer-name goblin \
#     --log-level DEBUG
