use alloy::providers::ProviderBuilder;
use constants::rpc_url;
use tokio;

pub mod constants;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let provider = ProviderBuilder::new().on_http(rpc_url());

    Ok(())
}
