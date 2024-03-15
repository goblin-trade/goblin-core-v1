# Steps

```sh
cargo build --target wasm32-unknown-unknown --release

cargo stylus check --endpoint http://localhost:8547
cargo stylus check --wasm-file-path ./target/wasm32-unknown-unknown/release/fairy_v1.wasm --endpoint http://localhost:8547

cargo stylus deploy --private-key-path ./.localnet.key --endpoint http://localhost:8547

cast send 0xda52b25ddB0e3B9CC393b0690Ac62245Ac772527 0xe50c000000000000 --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY


# Debugging
cargo stylus replay --tx 0x6ba35c46a35ba5a9ddbf839bdbc90863921d7d4210497ffc4ccdd07fa7f688e3 --endpoint http://localhost:8547

cargo stylus trace --tx 0x89f684ddda3b525ce3f1bfb2ef47d99a4a382ba89901ee126b152b8bb6b57b9c --endpoint http://localhost:8547
```

# Endianness

- My custom code is in lower endian for understandibility.
- Alloy primitive types use big endian. Address `0x000...1` is encoded as `[1, 0, 0, ...]`

```rs
// This is in big endian
let bytes = [u8; 20] = Address::new(&slice).0;
```
