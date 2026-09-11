//! Deposit `$BASE_TOKEN` through the hardcoded
//! `Pair<HardcodedERC20(0), HardcodedERC20(1)>` market.
//!
//! Calldata layout (21 bytes):
//!
//! ```text
//! byte 0       HeaderFlags          = 0x00
//! byte 1       MarketCounts byte 0  = 0x00   (hardcoded slots 0/1 empty)
//! byte 2       MarketCounts byte 1  = 0x01   (hardcoded slot 2 = BASE/QUOTE)
//! byte 3       MarketHeader         = 0x01   (decode_deposit_amounts)
//! byte 4..12   Base deposit         = i64 LE lots
//! byte 12..20  Quote deposit        = 0 (i64 LE)
//! byte 20      MarketIndex          = 0
//! ```
//!
//! Run with:
//!
//! ```sh
//! cargo run -p goblin-scripts-rs --example deposit-base-token
//! ```

use std::env;

use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256, utils::parse_units},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolCall,
};
use eyre::{Result, WrapErr};

sol! {
    #[allow(missing_docs)]
    interface IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}

/// Hardcoded base market, `Pair<HardcodedERC20(0), HardcodedERC20(1)>`.
const MARKET_COUNTS: [u8; 2] = [0x00, 0x01];
const MARKET_INDEX: u8 = 0;

/// Token and market decimals.
const TOKEN_DECIMALS: u8 = 18;
const GOBLIN_DECIMALS: u32 = 6;
/// `ATOMS_PER_UNIT / BaseLotsPerBaseUnit` for the hardcoded market.
const ATOMS_PER_LOT: u64 = 1_000_000 / 100;

fn build_deposit_calldata(base_lots: i64) -> Vec<u8> {
    let mut calldata = vec![0x00]; // HeaderFlags
    calldata.extend_from_slice(&MARKET_COUNTS);
    calldata.push(0x01); // MarketHeader: decode_deposit_amounts
    calldata.extend_from_slice(&base_lots.to_le_bytes());
    calldata.extend_from_slice(&0i64.to_le_bytes());
    calldata.push(MARKET_INDEX);
    calldata
}

fn base_lots_for_amount(amount: &str) -> Result<i64> {
    let raw_atoms: U256 = parse_units(amount, TOKEN_DECIMALS)?.into();
    let atoms = raw_atoms / U256::from(10u64.pow(TOKEN_DECIMALS as u32 - GOBLIN_DECIMALS));
    let lots = atoms / U256::from(ATOMS_PER_LOT);
    lots.try_into().wrap_err("BASE_AMOUNT too large")
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url: url::Url = env::var("ETH_RPC_URL")?.parse()?;
    let private_key = env::var("PRIVATE_KEY")?;
    let contract: Address = env::var("CONTRACT")?.parse()?;
    let base_token: Address = env::var("BASE_TOKEN")?.parse()?;
    let base_amount = env::var("BASE_AMOUNT").unwrap_or_else(|_| "1".to_string());

    let base_lots = base_lots_for_amount(&base_amount)?;
    let raw_atoms: U256 = parse_units(&base_amount, TOKEN_DECIMALS)?.into();

    let signer: PrivateKeySigner = private_key.parse()?;
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect_http(rpc_url);

    // The contract pulls the deposit from the caller with `transferFrom`.
    let approve = IERC20::approveCall {
        spender: contract,
        amount: raw_atoms,
    }
    .abi_encode();

    let receipt = provider
        .send_transaction(
            TransactionRequest::default()
                .to(base_token)
                .input(Bytes::from(approve).into()),
        )
        .await?
        .get_receipt()
        .await?;
    if !receipt.status() {
        eyre::bail!("approve reverted");
    }

    let calldata = build_deposit_calldata(base_lots);
    println!(
        "Depositing {base_amount} BASE_TOKEN ({base_lots} lots) into {contract} with calldata 0x{}",
        alloy::primitives::hex::encode(&calldata)
    );

    let receipt = provider
        .send_transaction(
            TransactionRequest::default()
                .to(contract)
                .input(Bytes::from(calldata).into()),
        )
        .await?
        .get_receipt()
        .await?;
    if !receipt.status() {
        eyre::bail!("deposit reverted");
    }

    println!("Deposit confirmed. Tx: {:?}", receipt.transaction_hash);
    Ok(())
}
