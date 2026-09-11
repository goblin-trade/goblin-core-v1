//! Deposit ETH into the deployed `goblin-program` contract.
//!
//! Goblin reads calldata as a compact byte stream, not as Solidity ABI. The
//! top-level payload is laid out as:
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────────────┐
//! │ GlobalArgs                                                           │
//! │  ├─ HeaderFlags          (1 byte)                                    │
//! │  └─ GlobalHeader         (variable length, depends on the flags)     │
//! │       ├─ eth_out_due          (8 bytes, only if `withdraw_eth`)      │
//! │       ├─ custom_recipient     (20 bytes, only if flag set)           │
//! │       ├─ market_counts        (2 bytes, +4 if `process_dynamic_...`) │
//! │       └─ custom_erc20_list    (20 bytes per custom token)            │
//! └──────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! See `goblin-core/src/input_processor/header_flags/impl_fixed_decode.rs` for the
//! bit layout of `HeaderFlags` and
//! `goblin-core/src/input_processor/global_args/global_header/impl_variable_decode.rs`
//! for the order of the variable fields.
//!
//! To deposit ETH we only need to turn on the `read_msg_value` flag. That makes
//! `HostioFields::try_new` read `msg.value` into `HostioFields::msg_value`, which
//! is credited to the sender's ETH store during settlement. Every other flag
//! stays off, so the global header is just two zero bytes of market counts and
//! there are no custom ERC20 tokens to list.
//!
//! Required environment variables (provided by the nix dev shell / `flake.nix`):
//!
//! ```text
//! ETH_RPC_URL   e.g. http://127.0.0.1:8547
//! PRIVATE_KEY   0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
//! CONTRACT      e.g. 0x8888ef09a63b6328468fce63a09fc185de807722
//! ```
//!
//! Optional:
//!
//! ```text
//! ETH_AMOUNT    human readable ETH amount, defaults to "0.1"
//! ```
//!
//! Run with:
//!
//! ```sh
//! cargo run -p goblin-scripts-rs --example deposit-eth
//! ```

use std::env;

use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256, utils::parse_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use eyre::{Result, WrapErr};

/// `HeaderFlags` bit for reading `msg.value` via hostio.
///
/// Mirrors `byte_0 & 0b0000_0010` in
/// `goblin-core/src/input_processor/header_flags/impl_fixed_decode.rs`.
const HEADER_FLAG_READ_MSG_VALUE: u8 = 0b0000_0010;

/// Build the top-level calldata for an ETH-only deposit.
///
/// Layout produced:
///
/// ```text
/// byte 0    HeaderFlags  = 0b0000_0010 (read_msg_value)
/// byte 1-2  MarketCounts = 0x0000      (no hardcoded/custom markets)
/// ```
///
/// There is no `eth_out_due` (the `withdraw_eth` flag is off), no custom
/// recipient and no custom ERC20 list (`custom_erc20_count == 0`), so the
/// encoded size is exactly three bytes.
pub fn build_deposit_calldata() -> Vec<u8> {
    // [HeaderFlags, MarketCounts byte 0, MarketCounts byte 1]
    //
    // MarketCounts holds the hardcoded specs 0, 1 and 2. `process_dynamic_markets`
    // is off, so the remaining dynamic count bytes are absent from the payload.
    vec![HEADER_FLAG_READ_MSG_VALUE, 0b0000_0000, 0b0000_0000]
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url: url::Url = env::var("ETH_RPC_URL")
        .wrap_err("ETH_RPC_URL must be set")?
        .parse()
        .wrap_err("ETH_RPC_URL must be a valid URL")?;
    let private_key = env::var("PRIVATE_KEY").wrap_err("PRIVATE_KEY must be set")?;
    let contract_addr: Address = env::var("CONTRACT")
        .wrap_err("CONTRACT must be set")?
        .parse()
        .wrap_err("CONTRACT must be a valid address")?;

    let eth_amount = env::var("ETH_AMOUNT").unwrap_or_else(|_| "0.1".to_string());
    let eth_value: U256 = parse_units(&eth_amount, "ether")
        .wrap_err("ETH_AMOUNT must be a valid decimal ETH amount")?
        .into();

    let signer: PrivateKeySigner = private_key.parse().wrap_err("invalid PRIVATE_KEY")?;
    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

    let calldata = build_deposit_calldata();
    println!(
        "Depositing {} ETH into {contract_addr} with calldata 0x{}",
        eth_amount,
        alloy::primitives::hex::encode(&calldata)
    );

    let tx = TransactionRequest::default()
        .to(contract_addr)
        .value(eth_value)
        .input(Bytes::from(calldata).into());

    let pending = provider.send_transaction(tx).await?;
    let tx_hash = *pending.tx_hash();
    println!("Submitted deposit transaction: {tx_hash:?}");

    let receipt = pending.get_receipt().await?;
    if !receipt.status() {
        eyre::bail!("deposit transaction reverted: {tx_hash:?}");
    }

    println!("Deposit confirmed. Tx: {tx_hash:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_calldata_sets_read_msg_value_only() {
        let calldata = build_deposit_calldata();

        // read_msg_value on, every other flag off, zero market counts.
        assert_eq!(calldata, vec![0b0000_0010, 0x00, 0x00]);

        // Guard against accidentally setting other flag bits.
        assert_eq!(
            calldata[0] & HEADER_FLAG_READ_MSG_VALUE,
            HEADER_FLAG_READ_MSG_VALUE
        );
        assert_eq!(calldata[0] & !HEADER_FLAG_READ_MSG_VALUE, 0);
    }
}
