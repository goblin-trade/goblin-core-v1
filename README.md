# Rounding numbers

## Rounding up division

```
(numerator + denominator - 1) / denominator
```

Used in
- compute_fee()
- compute_fees_after_matching_concludes()

## Rounding down division

Divide like usual

# Steps

```sh
cargo build --target wasm32-unknown-unknown --release

cargo stylus check --endpoint http://localhost:8547
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_market.wasm --endpoint http://localhost:8547

cargo stylus deploy --private-key-path ./.localnet.key --endpoint http://localhost:8547

cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_market.wasm --private-key-path ./.localnet.key --endpoint http://localhost:8547

cast send 0xA6E41fFD769491a42A6e5Ce453259b93983a22EF 0xe50c000000000000 --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY


# Bulk
cargo build --target wasm32-unknown-unknown --release && cargo stylus deploy --private-key-path ./.localnet.key --endpoint http://localhost:8547

# Debugging
cargo stylus replay --tx 0x6ba35c46a35ba5a9ddbf839bdbc90863921d7d4210497ffc4ccdd07fa7f688e3 --endpoint http://localhost:8547

cargo stylus trace --tx 0x89f684ddda3b525ce3f1bfb2ef47d99a4a382ba89901ee126b152b8bb6b57b9c --endpoint http://localhost:8547
```

# Endianness

- Bitmaps have no endianness.

- Structs are encoded in big endian, with the first struct element at 0th index.

- Use big endian to match EVM's convention.

# Goblin factory

```sh
cargo build --target wasm32-unknown-unknown --release

cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_market.wasm --private-key-path ./.localnet.key --endpoint http://localhost:8547

# Prepare goblin-market tx_data
cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_market.wasm --dry-run --output-tx-data-to-dir ./crates/goblin-factory/src --mode deploy-only --private-key-path ./.localnet.key --endpoint http://localhost:8547

cargo build --target wasm32-unknown-unknown --release

cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_factory.wasm --private-key-path ./.localnet.key --endpoint http://localhost:8547

# works once- something got deployed. But no code
cast send 0xDB2D15a3EB70C347E0D2C2c7861cAFb946baAb48 "initializeMarket()" --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY
```

# Problem in localnet

- create2 works on testnet 1, problem on localnet
- update: dry run command removed in 0.5.0 CLI. TODO update factory with new bytes

```sh
cargo build --target wasm32-unknown-unknown --release

# Prepare goblin-market tx_data
cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_market.wasm --dry-run --output-tx-data-to-dir ./crates/goblin-factory/src --mode deploy-only --private-key-path ./.mainnet.key --endpoint https://stylusv2.arbitrum.io/rpc

cargo build --target wasm32-unknown-unknown --release

cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_factory.wasm --private-key-path ./.mainnet.key --endpoint https://stylus-testnet.arbitrum.io/rpc

# works once- something got deployed. But no code
cast send 0x665bC3e8596e36E47dD831D6A49Aa985f585E1dA "initializeMarket()" --rpc-url 'https://stylus-testnet.arbitrum.io/rpc' --private-key $PRIVATE_KEY
```

# Order in removals

Order removal is designed to minimize slot writes.

- If a `SlotRestingOrder` is closed, we don't write the cleared value to slot. Instead we just turn off the bit in BitmapGroup.
- If a `BitmapGroup` is closed, don't write the cleared value to slot. Instead remove its outer index from index list.
  - Bitmap group is not written to slot when
    1. The best price on the outermost group changes- handle by updating best market price
    2. The group becomes empty- handle by removing outer index from list
- If a `ListSlot` in the index list is closed, don't write the cleared value to slot. Instead just update other list slots and decrement count in `MarketState`

# Testnode credentials

- Pvt key: 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
- Address: 0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E

# goblin-core crate



```sh
nix -p pkg-config openssl
cargo install --force cargo-stylus

cargo build -p goblin-core --release --target wasm32-unknown-unknown
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --endpoint http://127.0.0.1:8547
cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --no-verify --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 --endpoint http://127.0.0.1:8547

cast call 7e32b54800705876d3b5cfbc7d9c226a211f7c1a 0x01 --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 --rpc-url http://127.0.0.1:8547

cast tx 0xa9dc574bbed633cd6241c0baabd5792f3a0ec871ffc3f7195db8df43b521f057 --rpc-url http://127.0.0.1:8547

# For counter

# Set count
cast send f5ffd11a55afd39377411ab9856474d2a7cb697e "setCount(uint256)" 69 --rpc-url http://127.0.0.1:8547 --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
cast send f5ffd11a55afd39377411ab9856474d2a7cb697e 0xd14e62b80000000000000000000000000000000000000000000000000000000000000069 --rpc-url http://127.0.0.1:8547 --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659

# Get count
cast call f5ffd11a55afd39377411ab9856474d2a7cb697e "getCount()" --rpc-url http://127.0.0.1:8547
cast call f5ffd11a55afd39377411ab9856474d2a7cb697e 0xa87d942c --rpc-url http://127.0.0.1:8547
```
