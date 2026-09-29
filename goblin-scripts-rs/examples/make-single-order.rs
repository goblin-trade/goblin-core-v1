//! Rest **one** order on a deployed `goblin-program` contract, in two
//! transactions:
//!
//! 1. **deposit** — approve and fund the maker's WBTC and USDC stores.
//! 2. **make** — rest a single order.
//!
//! This is the minimal counterexample to `make-order-book.rs`: if a single
//! order reverts too, the 99-order book is not the problem; if only the book
//! reverts, the cause is scale or the bid side, not the calldata layout.
//!
//! Both payloads are built with `goblin-core`'s writer, i.e. the same header
//! types the contract decodes.
//!
//! # Which side?
//!
//! `SIDE` defaults to `ask`, which is valid on a *fresh* market and therefore
//! should succeed. A `bid` only rests once the market's last positions sit at
//! or above the tick: a fresh market has `last_positions = (0, 0)`, which puts
//! every tick in the quote region, and the contract rejects bids
//! (`InvalidOpenPrice`). On chain, last positions only move through takes, so
//! `SIDE=bid` reverts until the market has been primed by a take.
//!
//! # Gas
//!
//! A revert whose calldata is empty (`data: "0x"`) is the contract returning an
//! error, not an out-of-gas failure — an out-of-gas failure reports "out of
//! gas". One make writes a handful of slots; 99 writes are more, but still far
//! below a normal localnet block limit. If a *deployed* Stylus contract is not
//! cached, its first invocation also pays for compilation
//! (`cargo stylus cache bid <contract> 0`), which can dwarf the slot writes.
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
//! Optional:
//!
//! ```text
//! SIDE          "ask" (default) or "bid"
//! TICK          price in ticks, defaults to 8_000_000 (the 80_000 USD/BTC mid)
//! ```
//!
//! Run with:
//!
//! ```sh
//! cargo run -p goblin-scripts-rs --example make-single-order
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
        occupancy::OccupancyEnum,
        token::{HardcodedERC20, TokenDataTriple, token_list::CustomERC20List},
    },
    codec::{GoblinWrite, Writer},
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, INPUT_SIZE, MarketCounts, MarketCountsInner,
    },
    market::{InnerBitmapHeader, MakeHeader, MarketHeader, OuterBitmapHeader},
    quantities::{BaseLots, InnerPos, OuterBitmapIndexU32, OuterPos, UnsidedAtoms, UnsidedLots},
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

/// Hardcoded market parameters: `100` base lots per WBTC, `100` quote lots per
/// USDC and one quote lot per WBTC per tick.
const BASE_LOTS_PER_UNIT: u32 = 100;
const QUOTE_LOTS_PER_UNIT: u32 = 100;
const TICK_SIZE: u32 = 1;

/// Tick whose price is the `80_000` USD/BTC mid (`1 * 8_000_000 / 100`).
const MID_TICK: u64 = 8_000_000;

/// The mid tick must price at exactly `80_000` quote units per WBTC.
const _: () = assert!(TICK_SIZE as u64 * MID_TICK / QUOTE_LOTS_PER_UNIT as u64 == 80_000);

/// Base lots in the single order.
const ORDER_LOTS: u32 = 1;

/// Up-front deposits. Both sides are funded so either `SIDE` works.
const BASE_DEPOSIT_LOTS: i64 = 10;
const QUOTE_DEPOSIT_LOTS: i64 = 200_000;

/// ERC20 decimals assumed for both hardcoded tokens (the localnet list uses 18).
const TOKEN_DECIMALS: u32 = 18;

/// Goblin normalises balances to `10^6` atoms per unit.
const GOBLIN_DECIMALS: u32 = 6;

/// `ATOMS_PER_UNIT / lots_per_unit`, in goblin atoms.
const ATOMS_PER_LOT: u64 = 1_000_000 / BASE_LOTS_PER_UNIT as u64;

// ---------------------------------------------------------------------------
// Calldata encoding
// ---------------------------------------------------------------------------

const fn full_pos_of(tick: u64) -> u64 {
    tick << 3
}

const fn inner_pos_of(full_pos: u64) -> u8 {
    (full_pos & 0xFF) as u8
}

const fn outer_pos_of(full_pos: u64) -> u8 {
    ((full_pos >> 8) & 0xFF) as u8
}

const fn outer_bitmap_index_of(full_pos: u64) -> u32 {
    (full_pos >> 16) as u32
}

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

/// A `MarketHeader` for the hardcoded WBTC/USDC market. When
/// `decode_deposit_amounts` is false the deposit lots are not encoded.
fn hardcoded_market_header(
    base_deposit_lots: i64,
    quote_deposit_lots: i64,
    outer_bitmap_count: u8,
    decode_deposit_amounts: bool,
) -> MarketHeader<(Hardcoded, Pair<HardcodedERC20, HardcodedERC20>)> {
    MarketHeader::<(Hardcoded, Pair<HardcodedERC20, HardcodedERC20>)> {
        decode_deposit_amounts,
        execute_takes: Pair::new(false, false),
        outer_bitmap_count,
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
    hardcoded_market_header(base_deposit_lots, quote_deposit_lots, 0, true)
        .to_writer(writer, ())
        .unwrap();

    writer.get_calldata().to_vec()
}

/// The make calldata for a single order.
///
/// One outer bitmap holding one inner bitmap holding one make.
fn build_single_make_calldata(tick: u64, base_lots: u32, inner_enum_raw: bool) -> Vec<u8> {
    let full_pos = full_pos_of(tick);

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    hardcoded_global_args().to_writer(writer, ()).unwrap();
    hardcoded_market_header(0, 0, 1, false)
        .to_writer(writer, ())
        .unwrap();

    OuterBitmapHeader {
        outer_bitmap_index_u32: OuterBitmapIndexU32::new(outer_bitmap_index_of(full_pos)),
        inner_bitmap_count: 1,
    }
    .to_writer(writer, ())
    .unwrap();

    InnerBitmapHeader {
        outer_pos: OuterPos::new(outer_pos_of(full_pos)),
        update_count: 1,
    }
    .to_writer(writer, ())
    .unwrap();

    MakeHeader {
        inner_pos: InnerPos::new(inner_pos_of(full_pos)),
        occupancy_enum: OccupancyEnum::Vacant,
        inner_enum_raw,
        base_lots_u32: BaseLots::new(base_lots),
    }
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

    let side = env::var("SIDE").unwrap_or_else(|_| "ask".to_string());
    let inner_enum_raw = match side.to_lowercase().as_str() {
        "ask" => true,
        "bid" => false,
        other => eyre::bail!("SIDE must be \"ask\" or \"bid\", got {other:?}"),
    };
    let tick: u64 = match env::var("TICK") {
        Ok(raw) => raw.parse().wrap_err("TICK must be a u64")?,
        Err(_) => MID_TICK,
    };

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

    // --- Transaction 2: make -------------------------------------------------
    let make_calldata = build_single_make_calldata(tick, ORDER_LOTS, inner_enum_raw);
    println!(
        "Making one {side} of {ORDER_LOTS} lots at tick {tick} (price {}) with calldata 0x{}",
        TICK_SIZE as u64 * tick / QUOTE_LOTS_PER_UNIT as u64,
        alloy::primitives::hex::encode(&make_calldata)
    );

    let make_receipt = provider
        .send_transaction(
            TransactionRequest::default()
                .to(contract)
                .input(Bytes::from(make_calldata).into()),
        )
        .await?
        .get_receipt()
        .await?;
    if !make_receipt.status() {
        eyre::bail!(
            "make reverted; run with SIDE=ask on an unprimed market (bids need a prior take)"
        );
    }
    println!("Make confirmed. Tx: {:?}", make_receipt.transaction_hash);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_calldata_is_21_bytes() {
        // 1 flags + 2 market counts + 1 market header + 1 locator + 2 * 8 lots.
        let calldata = build_deposit_calldata(BASE_DEPOSIT_LOTS, QUOTE_DEPOSIT_LOTS);
        assert_eq!(calldata.len(), 21);
    }

    #[test]
    fn make_calldata_is_17_bytes() {
        // 3 global + 2 market header + 5 outer bitmap + 2 inner bitmap + 5 make.
        let calldata = build_single_make_calldata(MID_TICK, ORDER_LOTS, true);
        assert_eq!(calldata.len(), 17);
        assert!(calldata.len() <= INPUT_SIZE);
    }
}
