cargo build -p goblin-core --release --target wasm32-unknown-unknown
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --endpoint http://127.0.0.1:8547
cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_core.wasm --no-verify --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659 --endpoint http://127.0.0.1:8547
