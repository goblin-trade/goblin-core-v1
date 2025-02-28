# Deploy create3 factory at nonce 1
cd goblin-localnet-scripts
forge script script/DeployCREATE3Factory.s.sol:DeployCREATE3Factory \
    --private-key $PRIVATE_KEY \
    --rpc-url $ETH_RPC_URL \
    --broadcast \
    --skip-simulation

# Base token at nonce 2
readonly BASE_TOKENS_TO_MINT=100000000000000000000
forge create \
    --private-key $PRIVATE_KEY \
    --broadcast \
    TestERC20 --constructor-args "Base" "BASE" $BASE_TOKENS_TO_MINT

# Quote token at nonce 3
readonly QUOTE_TOKENS_TO_MINT=100000000000000000000
forge create \
    --private-key $PRIVATE_KEY \
    --broadcast \
    TestERC20 --constructor-args "Quote" "QUOTE" $QUOTE_TOKENS_TO_MINT

cd ..

# Deploy goblin core
cargo build -p goblin-core --release --target wasm32-unknown-unknown
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --endpoint http://127.0.0.1:8547

# Compile init code
cargo run -p compile-contract --bin compile-contract

# Deploy goblin_core with CREATE3
cast send $CREATE3_FACTORY \
    "deploy(bytes32,bytes)" $GOBLIN_SALT 0x$(xxd -p target/wasm32-unknown-unknown/release/goblin_core.contract | tr -d '\n') \
    --private-key $PRIVATE_KEY

# Activate contract
cast send 0x0000000000000000000000000000000000000071 \
    "activateProgram(address)" $CONTRACT \
    --private-key $PRIVATE_KEY \
    --value 0.0001ether
