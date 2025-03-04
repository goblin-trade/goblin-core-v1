use std::time::Duration;

use alloy::{
    eips::BlockNumberOrTag,
    providers::{ext::DebugApi, Provider, ProviderBuilder},
    rpc::types::{
        trace::geth::{
            CallConfig, FlatCallConfig, GethDebugBuiltInTracerType, GethDebugTracerType,
            GethDebugTracingOptions,
        },
        BlockTransactionsKind,
    },
};
use constants::{contract_address, rpc_url};
use tokio::{self, time::sleep};

pub mod constants;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let provider = ProviderBuilder::new().on_http(rpc_url());

    let mut block_number = 10;

    let tracing_options = GethDebugTracingOptions::flat_call_tracer(FlatCallConfig::default());

    loop {
        let block_trace = provider
            .debug_trace_block_by_number(
                BlockNumberOrTag::Number(block_number),
                tracing_options.clone(),
            )
            .await?;

        for tx in block_trace {
            if let Some(success_tx) = tx.success() {
                if let Ok(localized_traces) = success_tx.clone().try_into_flat_call_frame() {
                    for localized_trace in localized_traces {
                        if let Some(call_action) = localized_trace.trace.action.as_call() {
                            if call_action.to == contract_address() {
                                println!(
                                    "block {:?}, tx index {:?}",
                                    localized_trace.block_number,
                                    localized_trace.transaction_position
                                );
                                println!("tx hash {:?}", localized_trace.transaction_hash);
                                println!("Action {:?}", call_action);
                            }
                        }
                    }
                }
            }
        }

        block_number += 1;
        sleep(Duration::from_secs(10)).await;
    }
}
