# Goblin core

# Robinhood Testnet

RPC: https://robinhood-testnet.drpc.org

| Name            | Address |
|-----------------|---------|
| Create3 factory | [0xda52b25ddB0e3B9CC393b0690Ac62245Ac772527](https://explorer.testnet.chain.robinhood.com/address/0xda52b25ddB0e3B9CC393b0690Ac62245Ac772527) |
| Base token      | [0x11B57FE348584f042E436c6Bf7c3c3deF171de49](https://explorer.testnet.chain.robinhood.com/address/0x11B57FE348584f042E436c6Bf7c3c3deF171de49) |
| Quote token     | [0x1294b86822ff4976BfE136cB06CF43eC7FCF2574](https://explorer.testnet.chain.robinhood.com/address/0x1294b86822ff4976BfE136cB06CF43eC7FCF2574) |
| Goblin          | [0x888853cf2e8e5aee7157d84a7c2c5c514c52be34](https://explorer.testnet.chain.robinhood.com/address/0x888853cf2e8e5aee7157d84a7c2c5c514c52be34) |

- 99 make orders: [0xc6482ee0312c69e495a9e061b405620f7e9e61ed228616539883d0e15928c9e5](https://explorer.testnet.chain.robinhood.com/tx/0xc6482ee0312c69e495a9e061b405620f7e9e61ed228616539883d0e15928c9e5)



# Revised data for testnet

- Address: 0x888853cf2e8e5aee7157d84a7c2c5c514c52be34
- Salt: 0x0000000000000000000000000000000000000000000000004000000000000cd5

# Scripts

```sh
cargo build -p goblin-program --target wasm32-unknown-unknown --release

# check
cargo stylus check --wasm-file ./target/wasm32-unknown-unknown/release/goblin_program.wasm

# Deploy locally built file without verification
cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_program.wasm --private-key $PRIVATE_KEY --no-verify

cargo stylus deploy --wasm-file ./target/wasm32-unknown-unknown/release/goblin_program.wasm --private-key $PRIVATE_KEY --no-verify --no-activate

# Cache
cargo stylus cache bid 525c2aba45f66987217323e8a05ea400c65d06dc 0 --private-key $PRIVATE_KEY

# Get init code
cargo stylus get-initcode --output init-code-cli.txt
```
