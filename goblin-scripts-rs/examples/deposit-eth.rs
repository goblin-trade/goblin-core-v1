use alloy::{
    network::EthereumWallet,
    primitives::{utils::parse_units, Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::{Result, WrapErr};
use goblin_sdk_rs::GoblinCalldataBuilder;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url: url::Url = env::var("ETH_RPC_URL")
        .wrap_err("ETH_RPC_URL must be set")?
        .parse()?;
    let signer: PrivateKeySigner = env::var("PRIVATE_KEY")
        .wrap_err("PRIVATE_KEY must be set")?
        .parse()?;
    let contract_addr: Address = env::var("CONTRACT")
        .wrap_err("CONTRACT must be set")?
        .parse()?;

    let eth_amount_str = env::var("ETH_AMOUNT").unwrap_or_else(|_| "0.1".to_string());
    let eth_value: U256 = parse_units(&eth_amount_str, "ether")?.into();

    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);

    // Build calldata: enable read_msg_value
    let mut builder = GoblinCalldataBuilder::new();
    builder.set_msg_value(true);
    let calldata = builder.build()?;

    // Send transaction with msg.value
    let tx = TransactionRequest::default()
        .to(contract_addr)
        .value(eth_value)
        .input(Bytes::from(calldata).into());

    let tx_hash = provider.send_transaction(tx).await?.watch().await?;
    println!("Deposited {} ETH. Tx: {:?}", eth_amount_str, tx_hash);

    Ok(())
}
