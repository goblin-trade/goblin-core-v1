use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    providers::{Provider, ProviderBuilder},
    rpc::types::BlockTransactionsKind,
};
use constants::rpc_url;
use tokio::{self, time::sleep};

pub mod constants;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let provider = ProviderBuilder::new().on_http(rpc_url());

    let mut block_number = 10;

    loop {
        if let Some(block_data) = provider
            .get_block_by_number(
                BlockNumberOrTag::Number(block_number),
                BlockTransactionsKind::Full,
            )
            .await?
        {
            println!("Block {} data {:#?}", block_number, block_data);
        }

        // match provider
        //     .get_block_by_number(
        //         BlockNumberOrTag::Number(block_number),
        //         BlockTransactionsKind::Full,
        //     )
        //     .await
        // {
        //     Ok(Some(block_data)) => println!("{:#?}", block_data),
        //     Ok(None) => println!("Block {} not found", block_number),
        //     Err(e) => println!("Error fetching block {}: {:?}", block_number, e),
        // }

        block_number += 1;
        sleep(Duration::from_secs(10)).await;
    }
}
