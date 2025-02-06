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

# goblin-core crate

```sh
nix -p pkg-config openssl
cargo install --force cargo-stylus

cargo build -p goblin-core --release --target wasm32-unknown-unknown
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --endpoint https://sepolia-rollup.arbitrum.io/rpc
```
