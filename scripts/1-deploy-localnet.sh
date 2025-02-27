# Deploy create3 factory at nonce 1
cd goblin-localnet-scripts
forge script lib/create3-factory/script/Deploy.s.sol:DeployScript \
    --private-key $PRIVATE_KEY \
    --rpc-url $ETH_RPC_URL \
    --broadcast \
    --skip-simulation

# Base token at nonce 2
readonly BASE_TOKENS_TO_MINT=1000000
forge create \
    --private-key $PRIVATE_KEY \
    --broadcast \
    TestERC20 --constructor-args "Base" "BASE" $BASE_TOKENS_TO_MINT

# Quote token at nonce 3
readonly QUOTE_TOKENS_TO_MINT=1000000
forge create \
    --private-key $PRIVATE_KEY \
    --broadcast \
    TestERC20 --constructor-args "Quote" "QUOTE" $QUOTE_TOKENS_TO_MINT

cd ..

# Deploy goblin core
cargo build -p goblin-core --release --target wasm32-unknown-unknown
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --endpoint http://127.0.0.1:8547
# cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --no-verify --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 --endpoint http://127.0.0.1:8547

# Compile init code
cargo run -p compile-contract --bin compile-contract

# Deploy goblin_core with CREATE3
cast send $CREATE3_FACTORY \
    "deploy(bytes32,bytes)" $GOBLIN_SALT 0x$(xxd -p target/wasm32-unknown-unknown/release/goblin_core.contract | tr -d '\n') \
    --private-key $PRIVATE_KEY


# # Deploy ERC20 and mint 10^7 tokens (10 lots)
# cd ../test-erc20
# forge create --rpc-url http://127.0.0.1:8547 \
#     --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 \
#     --broadcast \
#     TestERC20 --constructor-args "MyToken" "MTK" 10000000
# cd ../goblin-contracts
