#!/usr/bin/env bash

# To use nitro-devnode instead
# bash ./nitro-devnode/run-dev-node.sh --stylus

rm -rf /tmp/dev-test/
./nitro/target/bin/nitro --dev \
    --chain.dev-wallet.private-key ${PRIVATE_KEY#0x} \
    --node.vmtrace.tracername noop \
    --log-level DEBUG
