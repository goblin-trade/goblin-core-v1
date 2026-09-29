//! A large two-sided book on the hardcoded WBTC/USDC market.
//!
//! Unlike `test_make_two_sided_book.rs` (a dynamic `Pair<CustomERC20,
//! CustomERC20>` market read from calldata), this book trades the single
//! *hardcoded* `Pair<HardcodedERC20, HardcodedERC20>` market: the two hardcoded
//! ERC20s are assumed to be WBTC (base, index 0) and USDC (quote, index 1). The
//! market's tick size is one quote lot and a quote unit is `100` lots, so the
//! `80_000` USD/BTC mid is tick `8_000_000`.
//!
//! The maker deposits first and makes in a second `entrypoint()` call, so the
//! make call only locks funds that a previous call credited.
//!
//! ## Layout
//!
//! `50` bids sit below the mid and `49` asks at/above it, spread over three
//! inner bitmaps:
//!
//! * the 50 bids are 50 distinct price levels, `mid - 50 .. mid - 1`, filling
//!   the top of one inner bitmap and all of the next one down;
//! * the 49 asks are the 32 price levels `mid .. mid + 31`: the 17 nearest
//!   levels carry a second order.
//!
//! An inner bitmap exposes 32 distinct prices (a tick maps to `inner_pos = (tick
//! % 32) * 8`), so 99 orders cannot occupy 99 prices in three bitmaps. The extra
//! orders are stacked in the spare column slots of their price level, which the
//! shared [`crate::test_utils::write_makes`] does for repeated ticks.
//!
//! Three inner bitmaps is also the largest that fits: calldata is capped at
//! `INPUT_SIZE` (512) bytes and a make header is 5 bytes, so 99 orders leave room
//! for exactly three `InnerBitmapHeader`s (505 + 2 * 3 = 511 bytes).

use goblin_core::{
    axis::{
        leg::{Base, Pair, Quote},
        token::{HardcodedERC20, token_list::HARDCODED_ERC20_LIST},
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::INPUT_SIZE,
    quantities::{BaseLots, FullPos, IntoAbs, UnsidedLots},
    state::{Preimage, RestingOrderPreimage, StorePreimage},
    types::StoreReader,
};
use goblin_hostio::hostio_unsafe::set_test_args;

use crate::test_utils::{
    MSG_SENDER, MakeOrder, USDC_HARDCODED_INDEX, WBTC_HARDCODED_INDEX,
    WBTC_USDC_HARDCODED_BASE_LOTS_PER_UNIT, WBTC_USDC_HARDCODED_MID_TICK,
    WBTC_USDC_HARDCODED_QUOTE_LOTS_PER_UNIT, WBTC_USDC_HARDCODED_TICK_SIZE, full_pos_of,
    hardcoded_token_address, hardcoded_wbtc_usdc_global_args, hardcoded_wbtc_usdc_make_calldata,
    hardcoded_wbtc_usdc_market, hardcoded_wbtc_usdc_market_header, isolated, mock_transfer_from,
    outer_bitmap_index_of, outer_pos_of, reset_call_state, set_sender,
};

/// Base lots in each order.
const ORDER_LOTS: u32 = 1;

/// Bids placed below the mid.
const BID_COUNT: usize = 50;

/// Asks placed at or above the mid.
const ASK_COUNT: usize = 49;

/// Distinct ask price levels. The remaining asks reuse these price levels.
const ASK_LEVELS: u64 = 32;

/// WBTC deposited up front: enough to collateralise the 49 asks.
const BASE_DEPOSIT_LOTS: i64 = 100;

/// USDC deposited up front: enough to collateralise the 50 bids.
const QUOTE_DEPOSIT_LOTS: i64 = 4_000_000;

/// The mid price sits at tick [`WBTC_USDC_HARDCODED_MID_TICK`].
const MID: u64 = WBTC_USDC_HARDCODED_MID_TICK;

/// Quote lots locked by a bid of [`ORDER_LOTS`] at `tick`.
fn bid_quote_lots(tick: u64) -> u64 {
    WBTC_USDC_HARDCODED_TICK_SIZE as u64 * tick * ORDER_LOTS as u64
        / WBTC_USDC_HARDCODED_BASE_LOTS_PER_UNIT as u64
}

/// The book: 50 bids below the mid, then 49 asks at/above it.
///
/// Bids are ordered nearest-first and asks level-first, so the slice is grouped
/// contiguously by inner bitmap for [`crate::test_utils::write_makes`].
fn two_sided_orders() -> Vec<MakeOrder> {
    let mut orders = Vec::with_capacity(BID_COUNT + ASK_COUNT);

    for i in 0..BID_COUNT as u64 {
        orders.push(MakeOrder {
            tick: MID - 1 - i,
            base_lots: ORDER_LOTS,
            inner_enum_raw: false,
        });
    }

    // One ask per price level, then a second order on the nearest 17 levels.
    for i in 0..ASK_LEVELS {
        orders.push(MakeOrder {
            tick: MID + i,
            base_lots: ORDER_LOTS,
            inner_enum_raw: true,
        });
    }
    for i in 0..(ASK_COUNT as u64 - ASK_LEVELS) {
        orders.push(MakeOrder {
            tick: MID + i,
            base_lots: ORDER_LOTS,
            inner_enum_raw: true,
        });
    }

    orders
}

#[test]
fn test_make_order_book() {
    let _guard = isolated();

    let orders = two_sided_orders();
    assert_eq!(orders.len(), BID_COUNT + ASK_COUNT);

    // The whole book lives in one outer bitmap but three inner bitmaps; assert
    // the layout up front so it cannot silently drift.
    let outer_bitmap_indices: std::collections::BTreeSet<_> = orders
        .iter()
        .map(|order| outer_bitmap_index_of(full_pos_of(order.tick)))
        .collect();
    let outer_positions: std::collections::BTreeSet<_> = orders
        .iter()
        .map(|order| outer_pos_of(full_pos_of(order.tick)))
        .collect();
    assert_eq!(outer_bitmap_indices.len(), 1);
    assert_eq!(outer_positions.len(), 3);

    let market = hardcoded_wbtc_usdc_market();
    let market_key = market.market_key;
    let wbtc_address = hardcoded_token_address(WBTC_HARDCODED_INDEX);
    let usdc_address = hardcoded_token_address(USDC_HARDCODED_INDEX);

    // Seed the last positions to the mid, so ticks below it are the base region
    // (bids) and ticks at/above it the quote region (asks). A fresh market would
    // put every tick in the quote region and reject the bids.
    let mid_full_pos = full_pos_of(MID);
    let mut market_state = market_key.load();
    market_state.last_positions = Pair::new(FullPos::new(mid_full_pos), FullPos::new(mid_full_pos));
    market_key.store(&market_state);

    // --- Call 1: the maker funds both sides ---------------------------------
    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    hardcoded_wbtc_usdc_global_args()
        .to_writer(writer, ())
        .unwrap();
    hardcoded_wbtc_usdc_market_header(BASE_DEPOSIT_LOTS, QUOTE_DEPOSIT_LOTS, 0, true)
        .to_writer(writer, ())
        .unwrap();

    set_test_args(writer.get_calldata().to_vec());
    set_sender();
    mock_transfer_from(wbtc_address);
    mock_transfer_from(usdc_address);

    let deposit_result = entrypoint();
    assert!(
        matches!(deposit_result, Ok(())),
        "deposit: {deposit_result:?}"
    );

    // --- Call 2: the maker builds the book ----------------------------------
    reset_call_state();
    let make_calldata = hardcoded_wbtc_usdc_make_calldata(&orders);
    // 99 five-byte makes, one outer bitmap, three inner bitmaps and the global
    // args must fit in the 512-byte calldata window.
    assert_eq!(
        make_calldata.len(),
        3 + 2 + 5 + 2 * 3 + 5 * (BID_COUNT + ASK_COUNT)
    );
    assert!(make_calldata.len() <= INPUT_SIZE);
    set_test_args(make_calldata);
    set_sender();

    let make_result = entrypoint();
    assert!(matches!(make_result, Ok(())), "make: {make_result:?}");

    // Every quoted order became a distinct resting order at its position.
    let resting_order = |full_pos: u64| {
        RestingOrderPreimage::<Pair<HardcodedERC20, HardcodedERC20>> {
            market_key,
            position: FullPos::new(full_pos),
        }
        .hash()
        .load()
    };

    for i in 0..BID_COUNT as u64 {
        let order = resting_order(full_pos_of(MID - 1 - i));
        assert!(order.base_lots == BaseLots::new(ORDER_LOTS as u64));
    }
    for i in 0..ASK_LEVELS {
        // Every level has a first order at column 0 …
        let order = resting_order(full_pos_of(MID + i));
        assert!(order.base_lots == BaseLots::new(ORDER_LOTS as u64));
    }
    for i in 0..(ASK_COUNT as u64 - ASK_LEVELS) {
        // … and the 17 nearest levels carry a second order at column 1.
        let order = resting_order(full_pos_of(MID + i) | 1);
        assert!(order.base_lots == BaseLots::new(ORDER_LOTS as u64));
    }

    let atoms_per_lot_pair = market.market.atoms_per_lot_pair();
    let base_atoms_per_lot = Base::get(&atoms_per_lot_pair);
    let quote_atoms_per_lot = Quote::get(&atoms_per_lot_pair);

    // WBTC store: the 49 asks each lock one lot of base.
    let base_store = StorePreimage::<HardcodedERC20> {
        trader: MSG_SENDER,
        token_address: wbtc_address,
    }
    .hash()
    .load();

    let base_locked = UnsidedLots::new(ASK_COUNT as u64 * ORDER_LOTS as u64) * base_atoms_per_lot;
    let base_deposit = UnsidedLots::new(BASE_DEPOSIT_LOTS).abs() * base_atoms_per_lot;

    assert_eq!(base_store.atoms_free, base_deposit - base_locked);
    assert_eq!(base_store.atoms_locked, base_locked);
    assert_eq!(
        base_store.decimals,
        HARDCODED_ERC20_LIST.inner[WBTC_HARDCODED_INDEX].decimals
    );

    // USDC store: each bid locks `price * size` quote lots, so nearer (higher)
    // bids lock more.
    let quote_store = StorePreimage::<HardcodedERC20> {
        trader: MSG_SENDER,
        token_address: usdc_address,
    }
    .hash()
    .load();

    let quote_locked_lots: u64 = (0..BID_COUNT as u64)
        .map(|i| bid_quote_lots(MID - 1 - i))
        .sum();
    let quote_locked = UnsidedLots::new(quote_locked_lots) * quote_atoms_per_lot;
    let quote_deposit = UnsidedLots::new(QUOTE_DEPOSIT_LOTS).abs() * quote_atoms_per_lot;

    assert_eq!(quote_store.atoms_free, quote_deposit - quote_locked);
    assert_eq!(quote_store.atoms_locked, quote_locked);
    assert_eq!(
        quote_store.decimals,
        HARDCODED_ERC20_LIST.inner[USDC_HARDCODED_INDEX].decimals
    );

    // Sanity-check the calibration: the best ask is 80_000 USD/BTC.
    assert_eq!(
        WBTC_USDC_HARDCODED_TICK_SIZE as u64 * WBTC_USDC_HARDCODED_MID_TICK
            / WBTC_USDC_HARDCODED_QUOTE_LOTS_PER_UNIT as u64,
        80_000
    );
}
