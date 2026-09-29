//! Rest the exact 50-bid / 49-ask book **from the integration test** on a
//! deployed `goblin-program` contract.
//!
//! Unlike `make-order-book.rs`, the make calldata is not re-encoded here:
//! `MAKE_CALLDATA` is the byte vector printed by
//! `goblin-core/tests/make/test_make_order_book.rs` (the
//! `println!("order calldata {:?}", make_calldata)` line), pasted verbatim.
//!
//! This isolates the calldata from the encoder: if the transaction still
//! reverts, the bytes are byte-for-byte the ones the integration test executes
//! successfully in the VM, so the difference is the on-chain market state, not
//! the encoding.
//!
//! The deposit is still encoded with `goblin-core`, like the other examples.
//!
//! # Prerequisite: the market must be primed
//!
//! A fresh market has `last_positions = (0, 0)`, which puts every tick in the
//! quote region, so the contract rejects the bids (`InvalidOpenPrice`). On chain
//! the last positions only move through takes, so prime the market (a take
//! against the ask side) before this make, or expect it to revert.
//!
//! Required environment variables:
//!
//! ```text
//! ETH_RPC_URL   e.g. http://127.0.0.1:8547
//! PRIVATE_KEY   0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
//! CONTRACT      deployed goblin-program address
//! BASE_TOKEN    WBTC address (hardcoded token 0)
//! QUOTE_TOKEN   USDC address (hardcoded token 1)
//! ```
//!
//! Run with:
//!
//! ```sh
//! cargo run -p goblin-scripts-rs --example make-order-book-raw
//! ```

use std::env;

use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolCall,
};
use eyre::{Result, WrapErr};
use goblin_core::{
    axis::{
        leg::Pair,
        market::{Hardcoded, MarketIndex},
        token::{HardcodedERC20, TokenDataTriple, token_list::CustomERC20List},
    },
    codec::{GoblinWrite, Writer},
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, INPUT_SIZE, MarketCounts, MarketCountsInner,
    },
    market::MarketHeader,
    quantities::{UnsidedAtoms, UnsidedLots},
    types::SameTriple,
};

sol! {
    #[allow(missing_docs)]
    interface IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
    }
}

/// The locator for the single hardcoded WBTC/USDC market.
const MARKET_INDEX: usize = 0;

/// Hardcoded market parameters: `100` base lots per WBTC and `100` quote lots
/// per USDC and one quote lot per WBTC per tick.
const BASE_LOTS_PER_UNIT: u32 = 100;

/// Up-front deposits, matching `test_make_order_book`.
const BASE_DEPOSIT_LOTS: i64 = 100;
const QUOTE_DEPOSIT_LOTS: i64 = 4_000_000;

/// ERC20 decimals assumed for both hardcoded tokens (the localnet list uses 18).
const TOKEN_DECIMALS: u32 = 18;

/// Goblin normalises balances to `10^6` atoms per unit.
const GOBLIN_DECIMALS: u32 = 6;

/// `ATOMS_PER_UNIT / lots_per_unit`, in goblin atoms.
const ATOMS_PER_LOT: u64 = 1_000_000 / BASE_LOTS_PER_UNIT as u64;

/// The make calldata printed by `test_make_order_book`, pasted verbatim.
///
/// 3 bytes of global args + 2 of market header + 5 of outer bitmap header +
/// 3 * 2 of inner bitmap headers + 99 * 5 of make headers = 511 bytes.
const MAKE_CALLDATA: [u8; 511] = [
    0, 0, 1, 8, 0, 208, 3, 0, 0, 3, 143, 32, 248, 4, 0, 0, 0, 240, 4, 0, 0, 0, 232, 4, 0, 0, 0,
    224, 4, 0, 0, 0, 216, 4, 0, 0, 0, 208, 4, 0, 0, 0, 200, 4, 0, 0, 0, 192, 4, 0, 0, 0, 184, 4, 0,
    0, 0, 176, 4, 0, 0, 0, 168, 4, 0, 0, 0, 160, 4, 0, 0, 0, 152, 4, 0, 0, 0, 144, 4, 0, 0, 0, 136,
    4, 0, 0, 0, 128, 4, 0, 0, 0, 120, 4, 0, 0, 0, 112, 4, 0, 0, 0, 104, 4, 0, 0, 0, 96, 4, 0, 0, 0,
    88, 4, 0, 0, 0, 80, 4, 0, 0, 0, 72, 4, 0, 0, 0, 64, 4, 0, 0, 0, 56, 4, 0, 0, 0, 48, 4, 0, 0, 0,
    40, 4, 0, 0, 0, 32, 4, 0, 0, 0, 24, 4, 0, 0, 0, 16, 4, 0, 0, 0, 8, 4, 0, 0, 0, 0, 4, 0, 0, 0,
    142, 18, 248, 4, 0, 0, 0, 240, 4, 0, 0, 0, 232, 4, 0, 0, 0, 224, 4, 0, 0, 0, 216, 4, 0, 0, 0,
    208, 4, 0, 0, 0, 200, 4, 0, 0, 0, 192, 4, 0, 0, 0, 184, 4, 0, 0, 0, 176, 4, 0, 0, 0, 168, 4, 0,
    0, 0, 160, 4, 0, 0, 0, 152, 4, 0, 0, 0, 144, 4, 0, 0, 0, 136, 4, 0, 0, 0, 128, 4, 0, 0, 0, 120,
    4, 0, 0, 0, 112, 4, 0, 0, 0, 144, 49, 0, 6, 0, 0, 0, 8, 6, 0, 0, 0, 16, 6, 0, 0, 0, 24, 6, 0,
    0, 0, 32, 6, 0, 0, 0, 40, 6, 0, 0, 0, 48, 6, 0, 0, 0, 56, 6, 0, 0, 0, 64, 6, 0, 0, 0, 72, 6, 0,
    0, 0, 80, 6, 0, 0, 0, 88, 6, 0, 0, 0, 96, 6, 0, 0, 0, 104, 6, 0, 0, 0, 112, 6, 0, 0, 0, 120, 6,
    0, 0, 0, 128, 6, 0, 0, 0, 136, 6, 0, 0, 0, 144, 6, 0, 0, 0, 152, 6, 0, 0, 0, 160, 6, 0, 0, 0,
    168, 6, 0, 0, 0, 176, 6, 0, 0, 0, 184, 6, 0, 0, 0, 192, 6, 0, 0, 0, 200, 6, 0, 0, 0, 208, 6, 0,
    0, 0, 216, 6, 0, 0, 0, 224, 6, 0, 0, 0, 232, 6, 0, 0, 0, 240, 6, 0, 0, 0, 248, 6, 0, 0, 0, 1,
    6, 0, 0, 0, 9, 6, 0, 0, 0, 17, 6, 0, 0, 0, 25, 6, 0, 0, 0, 33, 6, 0, 0, 0, 41, 6, 0, 0, 0, 49,
    6, 0, 0, 0, 57, 6, 0, 0, 0, 65, 6, 0, 0, 0, 73, 6, 0, 0, 0, 81, 6, 0, 0, 0, 89, 6, 0, 0, 0, 97,
    6, 0, 0, 0, 105, 6, 0, 0, 0, 113, 6, 0, 0, 0, 121, 6, 0, 0, 0, 129, 6, 0, 0, 0,
];

// ---------------------------------------------------------------------------
// Deposit calldata (encoded with goblin-core, like the other examples)
// ---------------------------------------------------------------------------

/// Global args for a call that processes only the hardcoded WBTC/USDC market.
fn hardcoded_global_args<'a>() -> GlobalArgs<'a> {
    GlobalArgs {
        flags: HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: false,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count: 0,
        },
        header: Header {
            eth_out_due_u32: UnsidedAtoms::default(),
            market_counts: MarketCounts::new(
                MarketCountsInner::new(
                    SameTriple::new(0, 0, 0), // base ETH
                    SameTriple::new(0, 1, 0), // base HardcodedERC20, quote HardcodedERC20
                    SameTriple::new(0, 0, 0), // base CustomERC20
                ),
                MarketCountsInner::default(), // dynamic markets
            ),
        },
        refs: HeaderRefs {
            custom_recipient: None,
            token_data_triple: TokenDataTriple::const_from(CustomERC20List { inner: &[] }),
        },
    }
}

/// A `MarketHeader` for the hardcoded WBTC/USDC market carrying the deposits.
fn hardcoded_market_header(
    base_deposit_lots: i64,
    quote_deposit_lots: i64,
) -> MarketHeader<(Hardcoded, Pair<HardcodedERC20, HardcodedERC20>)> {
    MarketHeader::<(Hardcoded, Pair<HardcodedERC20, HardcodedERC20>)> {
        decode_deposit_amounts: true,
        execute_takes: Pair::new(false, false),
        outer_bitmap_count: 0,
        local_deposits: Pair::new(
            UnsidedLots::new(base_deposit_lots),
            UnsidedLots::new(quote_deposit_lots),
        ),
        locator: MarketIndex::new(MARKET_INDEX),
    }
}

/// The deposit calldata: global args plus a market header carrying the local
/// deposits.
fn build_deposit_calldata(base_deposit_lots: i64, quote_deposit_lots: i64) -> Vec<u8> {
    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    hardcoded_global_args().to_writer(writer, ()).unwrap();
    hardcoded_market_header(base_deposit_lots, quote_deposit_lots)
        .to_writer(writer, ())
        .unwrap();

    writer.get_calldata().to_vec()
}

/// Raw ERC20 atoms for `lots` goblin lots: goblin normalises to `10^6` atoms
/// per unit, so the raw amount is scaled by `10^(decimals - 6)`.
fn raw_atoms_for_lots(lots: i64) -> U256 {
    U256::from(lots as u64)
        * U256::from(ATOMS_PER_LOT)
        * U256::from(10u64.pow(TOKEN_DECIMALS - GOBLIN_DECIMALS))
}

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url: url::Url = env::var("ETH_RPC_URL")
        .wrap_err("ETH_RPC_URL must be set")?
        .parse()
        .wrap_err("ETH_RPC_URL must be a valid URL")?;
    let private_key = env::var("PRIVATE_KEY").wrap_err("PRIVATE_KEY must be set")?;
    let contract: Address = env::var("CONTRACT")
        .wrap_err("CONTRACT must be set")?
        .parse()
        .wrap_err("CONTRACT must be a valid address")?;
    let base_token: Address = env::var("BASE_TOKEN")
        .wrap_err("BASE_TOKEN must be set")?
        .parse()
        .wrap_err("BASE_TOKEN must be a valid address")?;
    let quote_token: Address = env::var("QUOTE_TOKEN")
        .wrap_err("QUOTE_TOKEN must be set")?
        .parse()
        .wrap_err("QUOTE_TOKEN must be a valid address")?;

    let signer: PrivateKeySigner = private_key.parse().wrap_err("invalid PRIVATE_KEY")?;
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::from(signer))
        .connect_http(rpc_url);

    let base_atoms = raw_atoms_for_lots(BASE_DEPOSIT_LOTS);
    let quote_atoms = raw_atoms_for_lots(QUOTE_DEPOSIT_LOTS);

    // The contract pulls the deposit with `transferFrom`, so approve first.
    for (token, amount) in [(base_token, base_atoms), (quote_token, quote_atoms)] {
        let approve = IERC20::approveCall {
            spender: contract,
            amount,
        }
        .abi_encode();

        let receipt = provider
            .send_transaction(
                TransactionRequest::default()
                    .to(token)
                    .input(Bytes::from(approve).into()),
            )
            .await?
            .get_receipt()
            .await?;
        if !receipt.status() {
            eyre::bail!("approve reverted for {token}");
        }
    }

    // --- Transaction 1: deposit ---------------------------------------------
    let deposit_calldata = build_deposit_calldata(BASE_DEPOSIT_LOTS, QUOTE_DEPOSIT_LOTS);
    println!(
        "Depositing {BASE_DEPOSIT_LOTS} base lots and {QUOTE_DEPOSIT_LOTS} quote lots into {contract} with calldata 0x{}",
        alloy::primitives::hex::encode(&deposit_calldata)
    );

    let deposit_receipt = provider
        .send_transaction(
            TransactionRequest::default()
                .to(contract)
                .input(Bytes::from(deposit_calldata).into()),
        )
        .await?
        .get_receipt()
        .await?;
    if !deposit_receipt.status() {
        eyre::bail!("deposit reverted");
    }
    println!(
        "Deposit confirmed. Tx: {:?}",
        deposit_receipt.transaction_hash
    );

    // --- Transaction 2: make, with the test's byte-for-byte calldata ---------
    println!(
        "Sending the test's make calldata ({} bytes) 0x{}",
        MAKE_CALLDATA.len(),
        alloy::primitives::hex::encode(&MAKE_CALLDATA)
    );

    let make_receipt = provider
        .send_transaction(
            TransactionRequest::default()
                .to(contract)
                .input(Bytes::copy_from_slice(&MAKE_CALLDATA).into()),
        )
        .await?
        .get_receipt()
        .await?;
    if !make_receipt.status() {
        eyre::bail!(
            "make reverted; the calldata matches the test, so check the market state \
             (bids need a prior take to prime the market)"
        );
    }
    println!("Make confirmed. Tx: {:?}", make_receipt.transaction_hash);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_calldata_has_the_test_layout() {
        assert_eq!(MAKE_CALLDATA.len(), 511);

        // flags, market counts, market header (no deposits, 1 outer bitmap),
        // locator, then the outer bitmap index (976) little-endian.
        assert_eq!(&MAKE_CALLDATA[..5], &[0x00, 0x00, 0x01, 0x08, 0x00]);
        assert_eq!(&MAKE_CALLDATA[5..9], &[0xd0, 0x03, 0x00, 0x00]);

        // Checksum of the vector printed by `test_make_order_book`, so a
        // mistyped byte in the literal is caught.
        let sum: u32 = MAKE_CALLDATA.iter().map(|byte| *byte as u32).sum();
        assert_eq!(sum, 13_526);
    }

    #[test]
    fn deposit_calldata_is_21_bytes() {
        let calldata = build_deposit_calldata(BASE_DEPOSIT_LOTS, QUOTE_DEPOSIT_LOTS);
        assert_eq!(calldata.len(), 21);
    }
}
