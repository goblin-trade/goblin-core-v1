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

# copy_from_slice
# size 736, gas 1341202 - 0x13c680 = 45202
cast send 0x525c2aba45f66987217323e8a05ea400c65d06dc \
    0x0185d9a8a4bd77b9b5559c1b7fcb8ec9635922ed4985d9a8a4bd77b9b5559c1b7fcb8ec9635922ed490000000000000000 \
    --rpc-url http://127.0.0.1:8547 \
    --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659

# Using bytes directly
# size 736, gas 1341202 - 0x13c680
cast send 0x4af567288e68cad4aa93a272fe6139ca53859c70 \
    0x0185d9a8a4bd77b9b5559c1b7fcb8ec9635922ed4985d9a8a4bd77b9b5559c1b7fcb8ec9635922ed490000000000000000 \
    --rpc-url http://127.0.0.1:8547 \
    --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
```

# Optimization decisions

These have been benchmarked

- Avoid heap allocations. This costs less gas and allows us to get rid of the allocator. We need to use fixed size arrays, eg. `[u8; 512]` when dealing with arguments.
- Avoid zero filled arrays like `[0u8; 32]`. Zero filling increases size and gas cost. Instead use `Maybeuninit`.
- Prefer `[u64; 4]` over `[u8; 32]`. `u64` has an equivalent WASM opcode so we get a smaller binary.
- For getter functions, it is cheaper to accept arguments and hash inside the contract, rather than exposing a getter function for SLOAD where the hash must be found by the caller. This is because Solidity's memory cost grows exponentially while it grows linearly in stylus.
