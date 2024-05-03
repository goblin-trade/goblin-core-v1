# Steps

```sh
cargo build --target wasm32-unknown-unknown --release

cargo stylus check --endpoint http://localhost:8547
cargo stylus check --wasm-file-path ./target/wasm32-unknown-unknown/release/goblin_market.wasm --endpoint http://localhost:8547

cargo stylus deploy --private-key-path ./.localnet.key --endpoint http://localhost:8547

cargo stylus deploy --wasm-file-path ./target/wasm32-unknown-unknown/release/goblin_market.wasm --private-key-path ./.localnet.key --endpoint http://localhost:8547

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

cargo stylus deploy --wasm-file-path ./target/wasm32-unknown-unknown/release/goblin_market.wasm --private-key-path ./.localnet.key --endpoint http://localhost:8547

# works once- something got deployed
cast send 0xF5FfD11A55AFD39377411Ab9856474D2a7Cb697e "initializeMarket()" --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY

# activate
cargo stylus deploy --mode activate-only --activate-program-address 0xde1718dae23f8f1fd058279853fbb4fa11dc167e --wasm-file-path ./target/wasm32-unknown-unknown/release/goblin_market.wasm --private-key-path ./.localnet.key --endpoint http://localhost:8547
```