use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
};
use eyre::{Result, WrapErr};
use goblin_core_v1::quantities::{
    BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, UnsidedDeltaLots,
};
use goblin_sdk_rs::{GoblinCalldataBuilder, MarketCallBuilder, TokenKind};
use std::env;

sol! {
    #[sol(rpc)]
    interface IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}

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
    let base_token_addr: Address = env::var("BASE_TOKEN")
        .wrap_err("BASE_TOKEN must be set")?
        .parse()?;

    let deposit_lots: i64 = env::var("DEPOSIT_LOTS")
        .unwrap_or_else(|_| "10000".to_string())
        .parse()?;

    let wallet = EthereumWallet::from(signer);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(rpc_url);

    // 1. Serialize calldata using goblin-sdk-rs
    let mut builder = GoblinCalldataBuilder::new();
    let base_token_idx = builder.add_custom_token(base_token_addr.0 .0)?;

    let mut market = MarketCallBuilder::new();
    market
        .dynamic(
            TokenKind::CustomERC20,
            base_token_idx,
            TokenKind::ETH,
            0,
            BaseLotsPerBaseUnit::new(100),
            QuoteLotsPerQuoteUnit::new(1000),
            QuoteLotsPerBaseUnitPerTick::new(1),
        )?
        .deposit(
            UnsidedDeltaLots::new(deposit_lots),
            UnsidedDeltaLots::new(0),
        );

    builder.add_market(market.build()?);
    let calldata = builder.build()?;

    // 2. Approve $BASE_TOKEN for Goblin contract
    let erc20 = IERC20::new(base_token_addr, &provider);
    let approve_tx = erc20
        .approve(contract_addr, U256::MAX)
        .send()
        .await?
        .watch()
        .await?;
    println!("Approved $BASE_TOKEN. Tx: {:?}", approve_tx);

    // 3. Send deposit transaction to Goblin contract
    let tx = TransactionRequest::default()
        .to(contract_addr)
        .input(Bytes::from(calldata).into());

    let deposit_tx = provider.send_transaction(tx).await?.watch().await?;
    println!("Deposited {} base lots. Tx: {:?}", deposit_lots, deposit_tx);

    Ok(())
}
