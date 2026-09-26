//! Shared fixtures for the `goblin-core` integration tests.
//!
//! Test-specific hardcoded addresses, token ABIs and hostio mocks live here so
//! the integration tests do not each re-declare (and slowly drift from) the same
//! values.
//!
//! This module also owns the dynamic `Pair<CustomERC20, ETH>` market fixture
//! that the deposit and make tests all build.
//!
//! This is kept in `tests/test_utils/mod.rs` rather than `tests/test_utils.rs`
//! so Cargo does not compile it into its own empty test binary. Pull it into a
//! test binary with `mod test_utils;` (the test binaries that live in a
//! sub-directory have to include it by path, e.g.
//! `#[path = "../test_utils/mod.rs"] mod test_utils;`).

#![allow(dead_code)]

use std::sync::{Mutex, MutexGuard};

use alloy_sol_types::{SolCall, sol};
use goblin_core::{
    axis::{
        leg::Pair,
        market::Dynamic,
        occupancy::OccupancyEnum,
        token::{
            CustomERC20, CustomERC20Stub, ETH, ETHStub, TokenDataTriple,
            token_list::CustomERC20List,
            token_marker::{CustomERC20Index, TokenData},
        },
    },
    codec::{GoblinWrite, Writer},
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, MarketCounts, MarketCountsInner,
    },
    market::{CommonMarket, InnerBitmapHeader, MakeHeader, MarketHeader, OuterBitmapHeader},
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, InnerPos, OuterBitmapIndexU32, OuterPos,
        QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, UnsidedAtoms, UnsidedLots,
    },
    settlement::{ConstDefault, LocalDeposits, StaticDelta},
    types::SameTriple,
};
use goblin_hostio::hostio_unsafe::{
    VMContext, set_mock_call, set_mock_static_call, set_msg_sender, vm_ctx,
};
use hex_literal::hex;

/// The trader used as `msg_sender` by the integration tests.
pub const MSG_SENDER: [u8; 20] = hex!("11D05b50ac23f0F24F536315174f35E96d2D5354");

/// A second trader used as `msg_sender` by the take tests, so the taker is a
/// different party from the maker.
pub const TAKER: [u8; 20] = hex!("7D9a3AC8cAFe4A5c2D87d1a3c7F6C6b3E6A55C81");

/// A custom ERC20 token that is not part of the hardcoded token list, so it can
/// only be traded in a dynamic market. This is the mainnet USDC address, used
/// purely as an opaque address.
pub const CUSTOM_TOKEN: [u8; 20] = hex!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

/// Decimals mocked for [`CUSTOM_TOKEN`]. `update_erc20` accepts 6, 8 or 18.
pub const CUSTOM_TOKEN_DECIMALS: u8 = 6;

// The subset of the ERC20 ABI the tests exercise. Declared once here so the
// selectors and return encoders are derived rather than hand-rolled.
sol! {
    interface IERC20 {
        function transferFrom(address from, address to, uint256 amount) returns (bool);
        function decimals() external view returns (uint8);
    }
}

// ---------------------------------------------------------------------------
// Test isolation
// ---------------------------------------------------------------------------

/// Serialises tests that drive the process-global hostio and settlement state.
///
/// Every test in one integration-test binary shares a single process, so tests
/// that call `entrypoint()` would otherwise race on that shared state.
pub static SERIAL: Mutex<()> = Mutex::new(());

/// Clear the process-global hostio emulation and settlement scratch space.
///
/// [`VMContext`] and [`StaticDelta`] are process singletons: without this a test
/// would observe the storage, mocks, message fields and deltas left behind by a
/// sibling test.
pub fn reset_state() {
    *vm_ctx() = VMContext::new();
    *StaticDelta::get() = StaticDelta::DEFAULT;
}

/// Acquire [`SERIAL`] and clear the shared globals.
///
/// Hold the returned guard for the whole test so that sibling tests in the same
/// binary cannot interleave with it. Poisoning is ignored: a panicking test
/// should not wedge the rest of the suite.
pub fn isolated() -> MutexGuard<'static, ()> {
    let guard = SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    reset_state();
    guard
}

/// Clear the transient per-call state while keeping the on-chain storage.
///
/// `entrypoint()` leaves scratch state behind: the settled global delta, the
/// calldata and the message fields. A test that makes in one call and then takes
/// in a later call has to clear that scratch so the second call starts fresh,
/// but it must keep the storage that holds the resting orders and balances.
///
/// Registered hostio mocks are intentionally kept: they are keyed by call and
/// can be accumulated once per test.
pub fn reset_call_state() {
    *StaticDelta::get() = StaticDelta::DEFAULT;

    let ctx = vm_ctx();
    ctx.test_args.clear();
    ctx.test_result.clear();
    ctx.msg_value = [0u8; 32];
    ctx.msg_sender = [0u8; 20];
    ctx.last_return_data.clear();
    ctx.block_number = 0;
    ctx.block_timestamp = 0;
    ctx.msg_reentrant = false;
}

// ---------------------------------------------------------------------------
// Hostio mocks
// ---------------------------------------------------------------------------

/// Select [`MSG_SENDER`] as the caller for the next `entrypoint()` run.
pub fn set_sender() {
    set_msg_sender(MSG_SENDER);
}

/// Select [`TAKER`] as the caller for the next `entrypoint()` run.
pub fn set_taker() {
    set_msg_sender(TAKER);
}

/// Mock `token.transferFrom(..)` for `token` to succeed.
pub fn mock_transfer_from(token: [u8; 20]) {
    set_mock_call(
        token,
        IERC20::transferFromCall::SELECTOR.to_vec(),
        IERC20::transferFromCall::abi_encode_returns(&true),
    );
}

/// Mock `token.decimals()` for `token` to return `decimals`.
pub fn mock_decimals(token: [u8; 20], decimals: u8) {
    set_mock_static_call(
        token,
        IERC20::decimalsCall::SELECTOR.to_vec(),
        IERC20::decimalsCall::abi_encode_returns(&decimals),
    );
}

/// Mock every hostio read the custom ERC20 market performs on [`CUSTOM_TOKEN`]:
/// `decimals()` and `transferFrom(..)`.
pub fn mock_custom_token() {
    mock_decimals(CUSTOM_TOKEN, CUSTOM_TOKEN_DECIMALS);
    mock_transfer_from(CUSTOM_TOKEN);
}

// ---------------------------------------------------------------------------
// Position layout helpers
// ---------------------------------------------------------------------------
//
// A full position is 64 bits: 8 bits of inner pos (3 of column, 5 of row), 8 of
// outer pos and 48 of outer bitmap index. A tick is the full position shifted
// right by the 3 column bits, i.e. `full_pos = tick << 3`, so one outer bitmap
// spans `1 << 13` consecutive ticks. These helpers expose the decomposition that
// the bitmap headers carry.

/// The full position of `tick`.
pub const fn full_pos_of(tick: u64) -> u64 {
    tick << 3
}

/// Inner pos: the low 8 bits of the full position.
pub const fn inner_pos_of(full_pos: u64) -> u8 {
    (full_pos & 0xFF) as u8
}

/// Outer pos: bits 8..16 of the full position.
pub const fn outer_pos_of(full_pos: u64) -> u8 {
    ((full_pos >> 8) & 0xFF) as u8
}

/// Outer bitmap index: bits 16..64 of the full position.
pub const fn outer_bitmap_index_of(full_pos: u64) -> u32 {
    (full_pos >> 16) as u32
}

// ---------------------------------------------------------------------------
// Make payload writer
// ---------------------------------------------------------------------------
//
// The make instruction drives three nested headers after the market header:
// one `OuterBitmapHeader` per traversed outer bitmap, one `InnerBitmapHeader`
// per occupied outer position, and one `MakeHeader` per update. These helpers
// emit that payload so the make and take tests share one implementation.

/// A single resting order to place with a make.
pub struct MakeOrder {
    /// Price, in ticks. `full_pos = tick << 3`.
    pub tick: u64,

    /// Base lots offered.
    pub base_lots: u32,

    /// Which leg the maker locks, from the taker's perspective:
    /// `false` locks quote (a bid), `true` locks base (an ask).
    pub inner_enum_raw: bool,
}

/// Number of `OuterBitmapHeader`s [`write_makes`] emits for `orders`.
///
/// Orders must be grouped contiguously by outer bitmap index.
pub fn make_outer_bitmap_count(orders: &[MakeOrder]) -> u8 {
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

/// Write the make payload for `orders`.
///
/// Orders must be grouped contiguously by `(outer bitmap index, outer pos)`: a
/// new group opens a new inner bitmap, and a new outer bitmap index opens a new
/// outer bitmap. Within a group the makes keep their slice order.
pub fn write_makes(writer: &mut Writer<'_>, orders: &[MakeOrder]) {
    let mut outer_start = 0;

    while outer_start < orders.len() {
        let outer_bitmap_index = outer_bitmap_index_of(full_pos_of(orders[outer_start].tick));

        // End of this outer bitmap: the first order with a different index.
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

        // Emit each inner bitmap group followed by its makes.
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

            for order in &orders[inner_start..inner_end] {
                MakeHeader {
                    inner_pos: InnerPos::new(inner_pos_of(full_pos_of(order.tick))),
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

// ---------------------------------------------------------------------------
// Dynamic `Pair<CustomERC20, ETH>` market
// ---------------------------------------------------------------------------
//
// A custom ERC20 can never live in a hardcoded market, so it is listed in
// calldata and traded through a dynamic market. The deposit and make tests both
// build this same market, so its construction is centralised here.
//

/// The calldata token-list entry for [`CUSTOM_TOKEN`].
pub fn custom_token_data() -> TokenData<CustomERC20> {
    TokenData::<CustomERC20> {
        address: CUSTOM_TOKEN,
        decimals: CustomERC20Stub,
    }
}

/// Backing store for the calldata custom-token list.
///
/// [`CustomERC20List`] borrows a slice, so the caller must keep this array alive
/// for at least as long as the [`TokenDataTriple`] built from it.
pub fn custom_erc20_list_inner() -> [TokenData<CustomERC20>; 1] {
    [custom_token_data()]
}

/// The [`TokenDataTriple`] for the dynamic custom-ERC20 market.
pub fn custom_market_token_data_triple<'a>(
    inner: &'a [TokenData<CustomERC20>],
) -> TokenDataTriple<'a> {
    TokenDataTriple::const_from(CustomERC20List { inner })
}

/// `MarketCounts` selecting a single dynamic `Pair<CustomERC20, ETH>` market.
///
/// Legs are indexed `[market][base][quote]`, each ordered by variant: markets
/// `[Hardcoded, Dynamic]`, tokens `[ETH, HardcodedERC20, CustomERC20]`.
pub fn custom_erc20_eth_market_counts() -> MarketCounts {
    MarketCounts::new(
        MarketCountsInner::default(), // hardcoded markets
        MarketCountsInner::new(
            SameTriple::new(0, 0, 0), // base ETH
            SameTriple::new(0, 0, 0), // base HardcodedERC20
            SameTriple::new(1, 0, 0), // base CustomERC20, quote ETH
        ),
    )
}

/// Global args for a call that processes only the dynamic custom-ERC20 market.
pub fn custom_erc20_eth_global_args<'a>(token_data_triple: TokenDataTriple<'a>) -> GlobalArgs<'a> {
    GlobalArgs {
        flags: HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: true,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count: 1,
        },
        header: Header {
            eth_out_due_u32: UnsidedAtoms::default(),
            market_counts: custom_erc20_eth_market_counts(),
        },
        refs: HeaderRefs {
            custom_recipient: None,
            token_data_triple,
        },
    }
}

/// The dynamic `Pair<CustomERC20, ETH>` market locator read from calldata.
///
/// The locator is not sent as a slot key; the contract derives the market key
/// from the token addresses, and tests do the same via
/// [`CommonMarket::get_preimage`].
pub fn custom_erc20_eth_market() -> CommonMarket<Pair<CustomERC20, ETH>> {
    CommonMarket::<Pair<CustomERC20, ETH>>::new(
        Pair::new(CustomERC20Index::from(0), ETHStub),
        Pair::new(
            BaseLotsPerBaseUnit::new(100),
            QuoteLotsPerQuoteUnit::new(100),
        ),
        QuoteLotsPerBaseUnitPerTick::new(1),
    )
}

/// A [`MarketHeader`] for the dynamic custom-ERC20 market.
///
/// `deposit_lots_delta` is the local base (custom ERC20) deposit; `outer_bitmap_count`
/// is the number of make/outer-bitmap headers that follow.
pub fn custom_erc20_eth_market_header(
    deposit_lots_delta: i64,
    outer_bitmap_count: u8,
) -> MarketHeader<(Dynamic, Pair<CustomERC20, ETH>)> {
    MarketHeader::<(Dynamic, Pair<CustomERC20, ETH>)> {
        decode_deposit_amounts: true,
        execute_takes: Pair::new(false, false),
        outer_bitmap_count,
        local_deposits: LocalDeposits::<Pair<CustomERC20, ETH>>::new(
            UnsidedLots::new(deposit_lots_delta),
            ETHStub,
        ),
        locator: custom_erc20_eth_market(),
    }
}

// ---------------------------------------------------------------------------
// Dynamic `Pair<CustomERC20, CustomERC20>` WBTC/USDC market
// ---------------------------------------------------------------------------
//
// A realistic two-custom-token market: WBTC as base, USDC as quote. Both are
// listed in calldata and traded through a dynamic market.
//
// Pricing is normalised: `price = tick_size * tick / quote_lots_per_unit`, so
// the constants below are chosen to put the mid price at 80_000 USD/BTC:
//
//   800 * 10_000 / 100 = 80_000

/// Mainnet WBTC address, used purely as an opaque address.
pub const WBTC_TOKEN: [u8; 20] = hex!("2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599");

/// Decimals mocked for [`WBTC_TOKEN`]. `update_erc20` accepts 6, 8 or 18.
pub const WBTC_DECIMALS: u8 = 8;

/// Mainnet USDC address, used purely as an opaque address.
pub const USDC_TOKEN: [u8; 20] = hex!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

/// Decimals mocked for [`USDC_TOKEN`]. `update_erc20` accepts 6, 8 or 18.
pub const USDC_DECIMALS: u8 = 6;

/// Base lots per WBTC. `100` means one lot is `0.01` WBTC.
pub const WBTC_LOTS_PER_UNIT: u32 = 100;

/// Quote lots per USD. `100` means one lot is `0.01` USD.
pub const USDC_LOTS_PER_UNIT: u32 = 100;

/// Quote lots per WBTC per tick, i.e. `800 / 100 = 8` USD per tick.
pub const WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK: u32 = 800;

/// Tick whose price is the `80_000` USD/BTC mid.
pub const WBTC_USDC_MID_TICK: u64 = 10_000;

/// The calldata token list for [`WBTC_TOKEN`] (index 0) and [`USDC_TOKEN`]
/// (index 1).
pub fn wbtc_usdc_list_inner() -> [TokenData<CustomERC20>; 2] {
    [
        TokenData::<CustomERC20> {
            address: WBTC_TOKEN,
            decimals: CustomERC20Stub,
        },
        TokenData::<CustomERC20> {
            address: USDC_TOKEN,
            decimals: CustomERC20Stub,
        },
    ]
}

/// The [`TokenDataTriple`] for the WBTC/USDC dynamic market.
pub fn wbtc_usdc_token_data_triple<'a>(inner: &'a [TokenData<CustomERC20>]) -> TokenDataTriple<'a> {
    TokenDataTriple::const_from(CustomERC20List { inner })
}

/// `MarketCounts` selecting a single dynamic `Pair<CustomERC20, CustomERC20>`
/// market with WBTC as base and USDC as quote.
pub fn wbtc_usdc_market_counts() -> MarketCounts {
    MarketCounts::new(
        MarketCountsInner::default(), // hardcoded markets
        MarketCountsInner::new(
            SameTriple::new(0, 0, 0), // base ETH
            SameTriple::new(0, 0, 0), // base HardcodedERC20
            SameTriple::new(0, 0, 1), // base CustomERC20, quote CustomERC20
        ),
    )
}

/// Global args for a call that processes only the dynamic WBTC/USDC market.
pub fn wbtc_usdc_global_args<'a>(token_data_triple: TokenDataTriple<'a>) -> GlobalArgs<'a> {
    GlobalArgs {
        flags: HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: true,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count: 2,
        },
        header: Header {
            eth_out_due_u32: UnsidedAtoms::default(),
            market_counts: wbtc_usdc_market_counts(),
        },
        refs: HeaderRefs {
            custom_recipient: None,
            token_data_triple,
        },
    }
}

/// The dynamic `Pair<CustomERC20, CustomERC20>` WBTC/USDC market locator.
pub fn wbtc_usdc_market() -> CommonMarket<Pair<CustomERC20, CustomERC20>> {
    CommonMarket::<Pair<CustomERC20, CustomERC20>>::new(
        Pair::new(CustomERC20Index::from(0), CustomERC20Index::from(1)),
        Pair::new(
            BaseLotsPerBaseUnit::new(WBTC_LOTS_PER_UNIT),
            QuoteLotsPerQuoteUnit::new(USDC_LOTS_PER_UNIT),
        ),
        QuoteLotsPerBaseUnitPerTick::new(WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK),
    )
}

/// A [`MarketHeader`] for the WBTC/USDC dynamic market with both sides funded.
///
/// `execute_takes` selects which taker legs run: index 0 is `In = Base` (the
/// taker sells base into bids) and index 1 is `In = Quote` (the taker buys base
/// from asks).
pub fn wbtc_usdc_market_header_with_takes(
    base_deposit_lots: i64,
    quote_deposit_lots: i64,
    outer_bitmap_count: u8,
    execute_takes: Pair<bool, bool>,
) -> MarketHeader<(Dynamic, Pair<CustomERC20, CustomERC20>)> {
    MarketHeader::<(Dynamic, Pair<CustomERC20, CustomERC20>)> {
        decode_deposit_amounts: true,
        execute_takes,
        outer_bitmap_count,
        local_deposits: Pair::new(
            UnsidedLots::new(base_deposit_lots),
            UnsidedLots::new(quote_deposit_lots),
        ),
        locator: wbtc_usdc_market(),
    }
}

/// A make-only [`MarketHeader`] for the WBTC/USDC dynamic market.
pub fn wbtc_usdc_market_header(
    base_deposit_lots: i64,
    quote_deposit_lots: i64,
    outer_bitmap_count: u8,
) -> MarketHeader<(Dynamic, Pair<CustomERC20, CustomERC20>)> {
    wbtc_usdc_market_header_with_takes(
        base_deposit_lots,
        quote_deposit_lots,
        outer_bitmap_count,
        Pair::new(false, false),
    )
}

/// A take-only [`MarketHeader`] for the WBTC/USDC dynamic market: no makes
/// follow, and `execute_takes` selects the taker leg.
pub fn wbtc_usdc_take_market_header(
    base_deposit_lots: i64,
    quote_deposit_lots: i64,
    execute_takes: Pair<bool, bool>,
) -> MarketHeader<(Dynamic, Pair<CustomERC20, CustomERC20>)> {
    wbtc_usdc_market_header_with_takes(base_deposit_lots, quote_deposit_lots, 0, execute_takes)
}

/// Mock every hostio read the WBTC/USDC market performs: `decimals()` and
/// `transferFrom(..)` for both tokens.
pub fn mock_wbtc_usdc() {
    mock_decimals(WBTC_TOKEN, WBTC_DECIMALS);
    mock_transfer_from(WBTC_TOKEN);
    mock_decimals(USDC_TOKEN, USDC_DECIMALS);
    mock_transfer_from(USDC_TOKEN);
}
