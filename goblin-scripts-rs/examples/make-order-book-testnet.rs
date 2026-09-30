//! Rest the 50-bid / 49-ask WBTC/USDC book from
//! `goblin-core/tests/make/test_make_order_book.rs` on the deployed
//! `goblin-program` contract on Robinhood Testnet, in two transactions:
//!
//! 1. **deposit** — approve and fund the maker's WBTC and USDC stores.
//! 2. **make** — rest the book (50 bids below the mid, 49 asks at/above it).
//!
//! Both payloads are built with `goblin-core`'s writer, i.e. the same header
//! types (`GlobalArgs`, `MarketHeader`, `OuterBitmapHeader`,
//! `InnerBitmapHeader`, `MakeHeader`) the contract decodes, then submitted with
//! alloy — mirroring `deposit-erc20.rs`.
//!
//! This is a clone of `make-order-book.rs` whose RPC endpoint, contract and
//! token addresses are hardcoded from the Robinhood Testnet table in the
//! repository `README.md`, so only the signer key needs to be supplied.
//!
//! The market is the hardcoded `Pair<HardcodedERC20(0), HardcodedERC20(1)>`
//! market, assuming index 0 is WBTC (base) and index 1 is USDC (quote). The
//! `80_000` USD/BTC mid is tick `8_000_000`.
//!
//! # Prerequisite: the market must be primed
//!
//! The book only rests if the market's last positions already sit at/above the
//! mid. A fresh market has `last_positions = (0, 0)`, which puts *every* tick in
//! the quote region, so the contract rejects the bids (`InvalidOpenPrice`). On
//! chain the last positions only move through takes, so run a take against the
//! ask side first (priming the market) before running the make, or expect the
//! make transaction to revert.
//!
//! Required environment variables:
//!
//! ```text
//! PRIVATE_KEY   0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
//! ```
//!
//! Run with:
//!
//! ```sh
//! cargo run -p goblin-scripts-rs --example make-order-book-testnet
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

// ---------------------------------------------------------------------------
// Robinhood Testnet constants (from README.md)
// ---------------------------------------------------------------------------

/// Robinhood Testnet JSON-RPC endpoint.
const RPC_URL: &str = "https://robinhood-testnet.drpc.org";

/// The deployed `goblin-program` contract.
const CONTRACT: &str = "0x888853cf2e8e5aee7157d84a7c2c5c514c52be34";

/// Base token (hardcoded token 0).
const BASE_TOKEN: &str = "0x11B57FE348584f042E436c6Bf7c3c3deF171de49";

/// Quote token (hardcoded token 1).
const QUOTE_TOKEN: &str = "0x1294b86822ff4976BfE136cB06CF43eC7FCF2574";

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

/// Base lots in each order.
const ORDER_LOTS: u32 = 1;

/// Bids below the mid, asks at/above it, and the ask price levels used.
const BID_COUNT: u64 = 50;
const ASK_COUNT: u64 = 49;
const ASK_LEVELS: u64 = 32;

/// Up-front deposits: enough to collateralise both sides.
const BASE_DEPOSIT_LOTS: i64 = 100;
const QUOTE_DEPOSIT_LOTS: i64 = 4_000_000;

/// ERC20 decimals assumed for both hardcoded tokens (the localnet list uses 18).
const TOKEN_DECIMALS: u32 = 18;

/// Goblin normalises balances to `10^6` atoms per unit.
const GOBLIN_DECIMALS: u32 = 6;

/// `ATOMS_PER_UNIT / lots_per_unit`, in goblin atoms.
const ATOMS_PER_LOT: u64 = 1_000_000 / BASE_LOTS_PER_UNIT as u64;

// ---------------------------------------------------------------------------
// Calldata encoding
// ---------------------------------------------------------------------------

/// A make addressed by tick. Several orders may share a tick; they are stacked
/// in the spare column slots of that price level.
struct MakeOrder {
    tick: u64,
    base_lots: u32,
    inner_enum_raw: bool,
}

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

/// Number of `OuterBitmapHeader`s the make payload carries for `orders`.
fn outer_bitmap_count(orders: &[MakeOrder]) -> u8 {
    let mut count = 0u8;
    let mut last_index = None;

    for order in orders {
        let index = outer_bitmap_index_of(full_pos_of(order.tick));
        if last_index != Some(index) {
            count += 1;
            last_index = Some(index);
        }
    }

    count
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

/// Write the make payload for `orders`.
///
/// Orders must be grouped contiguously by `(outer bitmap index, outer pos)`: a
/// new group opens a new inner bitmap. A repeated tick is stacked into the next
/// free column of its price byte, so several orders rest at one price level.
fn write_makes(writer: &mut Writer<'_>, orders: &[MakeOrder]) {
    let mut outer_start = 0;

    while outer_start < orders.len() {
        let outer_bitmap_index = outer_bitmap_index_of(full_pos_of(orders[outer_start].tick));

        let mut outer_end = outer_start;
        while outer_end < orders.len()
            && outer_bitmap_index_of(full_pos_of(orders[outer_end].tick)) == outer_bitmap_index
        {
            outer_end += 1;
        }

        // Count the distinct (contiguous) outer positions in this outer bitmap.
        let mut inner_bitmap_count = 0u8;
        let mut last_outer_pos = None;
        for order in &orders[outer_start..outer_end] {
            let outer_pos = outer_pos_of(full_pos_of(order.tick));
            if last_outer_pos != Some(outer_pos) {
                inner_bitmap_count += 1;
                last_outer_pos = Some(outer_pos);
            }
        }

        OuterBitmapHeader {
            outer_bitmap_index_u32: OuterBitmapIndexU32::new(outer_bitmap_index),
            inner_bitmap_count,
        }
        .to_writer(writer, ())
        .unwrap();

        let mut inner_start = outer_start;
        while inner_start < outer_end {
            let outer_pos = outer_pos_of(full_pos_of(orders[inner_start].tick));
            let mut inner_end = inner_start;
            while inner_end < outer_end
                && outer_pos_of(full_pos_of(orders[inner_end].tick)) == outer_pos
            {
                inner_end += 1;
            }

            InnerBitmapHeader {
                outer_pos: OuterPos::new(outer_pos),
                update_count: (inner_end - inner_start) as u8,
            }
            .to_writer(writer, ())
            .unwrap();

            // One column counter per row of the inner bitmap.
            let mut columns = [0u8; 32];
            for order in &orders[inner_start..inner_end] {
                let row = (order.tick & 0x1F) as usize;
                let inner_pos = inner_pos_of(full_pos_of(order.tick)) | columns[row];
                columns[row] += 1;

                MakeHeader {
                    inner_pos: InnerPos::new(inner_pos),
                    occupancy_enum: OccupancyEnum::Vacant,
                    inner_enum_raw: order.inner_enum_raw,
                    base_lots_u32: BaseLots::new(order.base_lots),
                }
                .to_writer(writer, ())
                .unwrap();
            }

            inner_start = inner_end;
        }

        outer_start = outer_end;
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

/// The make calldata: global args plus a deposit-free market header and the make
/// payload for `orders`.
fn build_make_calldata(orders: &[MakeOrder]) -> Vec<u8> {
    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    hardcoded_global_args().to_writer(writer, ()).unwrap();
    hardcoded_market_header(0, 0, outer_bitmap_count(orders), false)
        .to_writer(writer, ())
        .unwrap();
    write_makes(writer, orders);

    writer.get_calldata().to_vec()
}

/// The book: 50 bids below the mid, then 49 asks at/above it.
fn two_sided_orders() -> Vec<MakeOrder> {
    let mut orders = Vec::with_capacity((BID_COUNT + ASK_COUNT) as usize);

    for i in 0..BID_COUNT {
        orders.push(MakeOrder {
            tick: MID_TICK - 1 - i,
            base_lots: ORDER_LOTS,
            inner_enum_raw: false,
        });
    }

    // One ask per price level, then a second order on the nearest 17 levels.
    for i in 0..ASK_LEVELS {
        orders.push(MakeOrder {
            tick: MID_TICK + i,
            base_lots: ORDER_LOTS,
            inner_enum_raw: true,
        });
    }
    for i in 0..(ASK_COUNT - ASK_LEVELS) {
        orders.push(MakeOrder {
            tick: MID_TICK + i,
            base_lots: ORDER_LOTS,
            inner_enum_raw: true,
        });
    }

    orders
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
    let rpc_url: url::Url = RPC_URL.parse().wrap_err("RPC_URL must be a valid URL")?;
    let private_key = env::var("PRIVATE_KEY").wrap_err("PRIVATE_KEY must be set")?;
    let contract: Address = CONTRACT
        .parse()
        .wrap_err("CONTRACT must be a valid address")?;
    let base_token: Address = BASE_TOKEN
        .parse()
        .wrap_err("BASE_TOKEN must be a valid address")?;
    let quote_token: Address = QUOTE_TOKEN
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

    // --- Transaction 2: make ------------------------------------------------
    let orders = two_sided_orders();
    let make_calldata = build_make_calldata(&orders);
    println!(
        "Resting {} orders ({} bids, {} asks) with calldata 0x{}",
        orders.len(),
        BID_COUNT,
        ASK_COUNT,
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
            "make reverted; is the market primed? a fresh market rejects the bid side \
             (run a take against the asks first)"
        );
    }
    println!("Make confirmed. Tx: {:?}", make_receipt.transaction_hash);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testnet_constants_are_valid_addresses() {
        assert!(CONTRACT.parse::<Address>().is_ok());
        assert!(BASE_TOKEN.parse::<Address>().is_ok());
        assert!(QUOTE_TOKEN.parse::<Address>().is_ok());
        assert!(RPC_URL.parse::<url::Url>().is_ok());
    }

    #[test]
    fn deposit_calldata_is_21_bytes() {
        // 1 flags + 2 market counts + 1 market header + 1 locator + 2 * 8 lots.
        let calldata = build_deposit_calldata(BASE_DEPOSIT_LOTS, QUOTE_DEPOSIT_LOTS);
        assert_eq!(calldata.len(), 21);
    }

    #[test]
    fn make_calldata_fits_the_window() {
        let orders = two_sided_orders();
        assert_eq!(orders.len(), (BID_COUNT + ASK_COUNT) as usize);

        // One outer bitmap, three inner bitmaps, 99 five-byte makes.
        assert_eq!(outer_bitmap_count(&orders), 1);
        let calldata = build_make_calldata(&orders);
        assert_eq!(
            calldata.len(),
            3 + 2 + 5 + 2 * 3 + 5 * (BID_COUNT + ASK_COUNT) as usize
        );
        assert!(calldata.len() <= INPUT_SIZE);
    }
}
