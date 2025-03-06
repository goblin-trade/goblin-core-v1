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

cast send 0xd92773693917f0ff664f85c3cb698c33420947ff \
    0x003f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E \
    --value 1wei \
    --rpc-url http://127.0.0.1:8547 \
    --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659

cast call 0x525c2aba45f66987217323e8a05ea400c65d06dc \
    0x0A3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E0000000000000000000000000000000000000000 \
    --rpc-url http://127.0.0.1:8547
```

# Tracing calls

## debug_traceTransaction
```sh
curl -X POST $ETH_RPC_URL \
  -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"debug_traceTransaction","params":["0x2d00227e53ffb6b7fbb165b1b5c1945c6a18db9dd4a669edc4fba6ac959ec9e6",{"tracer":"stylusTracer"}],"id":1}'
```

`stylusTracer` will return data in below format.

```sh
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "name": "user_entrypoint",
      "args": "0x00000032",
      "outs": "0x",
      "startInk": 4781120000,
      "endInk": 4781120000
    },
    {
      "name": "read_args",
      "args": "0x",
      "outs": "0x0101e1080224b632a93951a7cfa33eeea9fd81558b5e3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e0100000000000000",
      "startInk": 4781109267,
      "endInk": 4781095287
    },
    {
      "name": "msg_sender",
      "args": "0x",
      "outs": "0x3f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e",
      "startInk": 4781058849,
      "endInk": 4781045409
    },
    {
      "name": "call_contract",
      "args": "0xe1080224b632a93951a7cfa33eeea9fd81558b5e0000000000030d40000000000000000000000000000000000000000000000000000000000000000023b872dd0000000000000000000000003f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e0000000000000000000000008888415db80eabcf580283a3d65249887d3161b000000000000000000000000000000000000000000000000000000000000f4240",
      "outs": "0x0000002000",
      "startInk": 4780924782,
      "endInk": 4398231347,
      "address": "0xe1080224b632a93951a7cfa33eeea9fd81558b5e",
      "steps": []
    },
```

A successful call will return

```sh
    {
      "name": "user_returned",
      "args": "0x",
      "outs": "0x00000000",
      "startInk": 4781120000,
      "endInk": 4781120000
    }
```

## debug_traceBlockByNumber

```sh
curl -X POST $ETH_RPC_URL \
  -H "Content-Type: application/json" \
  --data '{"method":"debug_traceBlockByNumber","params":["0xa", {"tracer": "stylusTracer"}],"id":1,"jsonrpc":"2.0"}'

curl -X POST $ETH_RPC_URL \
  -H "Content-Type: application/json" \
  --data '{"method":"debug_traceBlockByNumber","params":["0xa", {"tracer": "callTracer"}],"id":1,"jsonrpc":"2.0"}'
```

This returns trace for each transaction in the block but doesn't contain contract address 0x8888...

## arbtrace endpoints

```sh
curl -X POST $ETH_RPC_URL \
  -H "Content-Type: application/json" \
  --data '{"method":"arbtrace_block","params":["0xc"],"id":1,"jsonrpc":"2.0"}'
```

## Using eth_getBlockByNumber instead of debug endpoints

Doen't provide information on internal calls.

```bash
curl -X POST $ETH_RPC_URL \
    -H "Content-Type: application/json" \
    --data '{"method":"eth_getBlockByNumber","params":["0xa",true],"id":1,"jsonrpc":"2.0"}'
```

## Batching multiple calls

This works. TODO call in alloy-rs in type safe way.

```bash
curl -X POST $ETH_RPC_URL \
  -H "Content-Type: application/json" \
  --data '[
    {"method":"debug_traceBlockByNumber","params":["0x9", {"tracer": "flatCallTracer"}],"id":1,"jsonrpc":"2.0"},
    {"method":"debug_traceBlockByNumber","params":["0xa", {"tracer": "flatCallTracer"}],"id":1,"jsonrpc":"2.0"}
  ]'
```

## Custom JS tracer

```js
{
  data: [],
  fault: function(log) {},
  step: function(log) { if(log.op.toString() == "CALL") this.data.push(log.stack.peek(0)); },
  result: function() { return this.data; }
}
```

```sh
curl -X POST $ETH_RPC_URL \
  -H "Content-Type: application/json" \
  --data '{
    "method": "debug_traceBlockByNumber",
    "params": [
      "0xa",
      {
        "tracer": "{data: [], fault: function(log) {}, step: function(log) { if(log.op.toString() == \"CALL\") this.data.push(log.stack.peek(0)); }, result: function() { return this.data; }}"
      }
    ],
    "id": 1,
    "jsonrpc": "2.0"
  }'

TRACER='{
    data: [],
    fault: function(log) {},
    step: function(log) {
        if(log.op.toString() == "CALL") {
            this.data.push(log.stack.peek(0));
        }
    },
    result: function() { return this.data; }
}'

curl -X POST "$ETH_RPC_URL" \
  -H "Content-Type: application/json" \
  --data "$(jq -n '{
    method: "debug_traceBlockByNumber",
    params: ["0xa", { tracer: $tracer }],
    id: 1,
    jsonrpc: "2.0"
  }' --arg tracer "$TRACER")"
```

### Filter to get traces to 0x8888415db80eabcf580283a3d65249887d3161b0

```sh
TRACER='{
    data: [],
    fault: function(log) {},
    step: function (log) {
        if (log.op.toString() === "CALL" || log.op.toString() === "CALLCODE") {
            let to = log.contract.getAddress(); // Use getTo() for the recipient
             this.data.push(log.stack.peek(0));
        }
    },
    result: function () {
        return this.data;
    }
}'

curl -X POST "$ETH_RPC_URL" \
  -H "Content-Type: application/json" \
  --data "$(jq -n '{
    method: "debug_traceBlockByNumber",
    params: ["0xa", { tracer: $tracer }],
    id: 1,
    jsonrpc: "2.0"
  }' --arg tracer "$TRACER")"
```

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "txHash": "0x4984f2c62714e25fdb8c4b47393f4545afcd00dca86f3cd05ab34d2db9687718",
      "result": [],
    },
    {
      "txHash": "0x2d00227e53ffb6b7fbb165b1b5c1945c6a18db9dd4a669edc4fba6ac959ec9e6",
      "result": [],
    },
  ],
}
```

# Optimization decisions

These have been benchmarked

- Avoid heap allocations. This costs less gas and allows us to get rid of the allocator. We need to use fixed size arrays, eg. `[u8; 512]` when dealing with arguments.
- Avoid zero filled arrays like `[0u8; 32]`. Zero filling increases size and gas cost. Instead use `Maybeuninit`.
- Prefer `[u64; 4]` over `[u8; 32]`. `u64` has an equivalent WASM opcode so we get a smaller binary.
- For getter functions, it is cheaper to accept arguments and hash inside the contract, rather than exposing a getter function for SLOAD where the hash must be found by the caller. This is because Solidity's memory cost grows exponentially while it grows linearly in stylus.
