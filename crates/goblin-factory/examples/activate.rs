//! Example on how to interact with a deployed `stylus-hello-world` program using defaults.
//! This example uses ethers-rs to instantiate the program using a Solidity ABI.
//! Then, it attempts to check the current counter value, increment it via a tx,
//! and check the value again. The deployed program is fully written in Rust and compiled to WASM
//! but with Stylus, it is accessible just as a normal Solidity smart contract is via an ABI.

use ethers::{
    middleware::SignerMiddleware,
    prelude::abigen,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Eip1559TransactionRequest, TransactionRequest, H160},
};
use eyre::eyre;
use hex;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;
use stylus_sdk::tx;

pub const ARB_WASM_ADDRESS: &str = "0000000000000000000000000000000000000071";

/// 4 bytes method selector for the activate method of ArbWasm.
pub const ARBWASM_ACTIVATE_METHOD_HASH: &str = "58c780c2";

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let privkey = "0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659";
    let wallet = LocalWallet::from_str(&privkey)?;
    println!("wallet {:?}", wallet.address());

    let provider = Provider::<Http>::try_from("http://localhost:8547")?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.clone().with_chain_id(chain_id),
    ));

    let program_addr = Address::from_str("0xde1718dae23f8f1fd058279853fbb4fa11dc167e")?;
    let activate_calldata = activation_calldata(&program_addr);

    let arb_wasm_address = Address::from_str("0x0000000000000000000000000000000000000071")?;
    // let arb_wasm_address = Address::from_str("0xde1718dae23f8f1fd058279853fbb4fa11dc167e")?;

    // let calldata = vec![0, 0, 0, 0, 0];

    let tx = TransactionRequest::new()
        .to(arb_wasm_address)
        .data(activate_calldata)
        // .data(calldata)
        .gas_price(100000000)
        .gas(100000000)
        .value(10);

    let tx_response = client.send_transaction(tx, None).await?;

    println!("Transaction sent. Hash: {:?}", tx_response);

    // let mut tx_request = Eip1559TransactionRequest::new()
    //     .from(wallet.address())
    //     .to(arb_wasm_address)
    //     .data(activate_calldata);

    // client.call(&tx_request, block);

    Ok(())
}

fn read_secret_from_file(fpath: &str) -> eyre::Result<String> {
    let f = std::fs::File::open(fpath)?;
    let mut buf_reader = BufReader::new(f);
    let mut secret = String::new();
    buf_reader.read_line(&mut secret)?;
    Ok(secret.trim().to_string())
}

pub fn activation_calldata(program_addr: &H160) -> Vec<u8> {
    let mut activate_calldata = vec![];
    let activate_method_hash = hex::decode(ARBWASM_ACTIVATE_METHOD_HASH).unwrap();
    activate_calldata.extend(activate_method_hash);
    let mut extension = [0u8; 32];
    // Next, we add the address to the last 20 bytes of extension
    extension[12..32].copy_from_slice(program_addr.as_bytes());
    activate_calldata.extend(extension);
    activate_calldata
}
