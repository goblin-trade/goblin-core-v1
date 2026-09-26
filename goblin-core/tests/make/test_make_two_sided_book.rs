//! A realistic two-sided book on a dynamic WBTC/USDC market.
//!
//! Base is WBTC and quote is USDC. The market is calibrated so the mid price is
//! `80_000` USD/BTC (`800 * 10_000 / 100`), and both sides are made around it:
//!
//! * bids (`In = Base`, locking quote) at ticks below the mid,
//! * asks (`In = Quote`, locking base) at ticks at or above the mid.
//!
//! All six orders land in a single inner bitmap: the mid tick sits at row 16 of
//! its 32-row inner bitmap, so the whole spread stays inside one
//! `InnerBitmapHeader`. The best bid and best ask are adjacent ticks, i.e. one
//! row apart.
//!
//! The region split comes from the market's last positions, which are seeded to
//! the mid. On a fresh market `last_positions = (0, 0)`, which puts every
//! position in the quote region and therefore rejects bids, so the seed is what
//! makes a two-sided book expressible through makes alone.

use goblin_core::{
    axis::{
        leg::{Base, Pair, Quote},
        token::CustomERC20,
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
    MSG_SENDER, MakeOrder, USDC_DECIMALS, USDC_LOTS_PER_UNIT, USDC_TOKEN, WBTC_DECIMALS,
    WBTC_LOTS_PER_UNIT, WBTC_TOKEN, WBTC_USDC_MID_TICK,
    WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK, full_pos_of, inner_pos_of, isolated,
    make_outer_bitmap_count, mock_wbtc_usdc, outer_bitmap_index_of, outer_pos_of, set_sender,
    wbtc_usdc_global_args, wbtc_usdc_list_inner, wbtc_usdc_market_header,
    wbtc_usdc_token_data_triple, write_makes,
};

/// Base lots in each order. One lot is `0.01` WBTC, so each order is `0.01` BTC.
const ORDER_LOTS: u64 = 1;

/// WBTC deposited up front: enough to collateralise the three asks.
const BASE_DEPOSIT_LOTS: i64 = 10;

/// USDC deposited up front: enough to collateralise the three bids.
const QUOTE_DEPOSIT_LOTS: i64 = 300_000;

/// Bid ticks, all below the mid. Placed nearest-first.
const BID_TICKS: [u64; 3] = [
    WBTC_USDC_MID_TICK - 3,
    WBTC_USDC_MID_TICK - 2,
    WBTC_USDC_MID_TICK - 1,
];

/// Ask ticks, all at or above the mid. Nearest-first.
const ASK_TICKS: [u64; 3] = [
    WBTC_USDC_MID_TICK,
    WBTC_USDC_MID_TICK + 1,
    WBTC_USDC_MID_TICK + 2,
];

/// Quote lots locked by a bid of [`ORDER_LOTS`] at `tick`.
///
/// `price_in_quote_lots = tick_size * tick`, and a quote bid locks
/// `price_in_quote_lots * base_lots / base_lots_per_unit`.
fn bid_quote_lots(tick: u64) -> u64 {
    WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK as u64 * tick * ORDER_LOTS
        / WBTC_LOTS_PER_UNIT as u64
}

#[test]
fn test_make_two_sided_book() {
    let _guard = isolated();

    let tokens = wbtc_usdc_list_inner();
    let token_data_triple = wbtc_usdc_token_data_triple(&tokens);

    // Bids buy base (taker leg base, `inner_enum_raw = false`, maker locks
    // quote); asks sell base (taker leg quote, `inner_enum_raw = true`, maker
    // locks base).
    let orders = [
        MakeOrder {
            tick: BID_TICKS[0],
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick: BID_TICKS[1],
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick: BID_TICKS[2],
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick: ASK_TICKS[0],
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: true,
        },
        MakeOrder {
            tick: ASK_TICKS[1],
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: true,
        },
        MakeOrder {
            tick: ASK_TICKS[2],
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: true,
        },
    ];

    let global_args = wbtc_usdc_global_args(token_data_triple);
    let market_header = wbtc_usdc_market_header(
        BASE_DEPOSIT_LOTS,
        QUOTE_DEPOSIT_LOTS,
        make_outer_bitmap_count(&orders),
    );

    let market_key = market_header
        .locator
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    // Seed the last positions to the mid. This is what splits the book into a
    // base region below the mid (bids) and a quote region at/above it (asks).
    let mid_full_pos = full_pos_of(WBTC_USDC_MID_TICK);
    let mut market_state = market_key.load();
    market_state.last_positions = Pair::new(FullPos::new(mid_full_pos), FullPos::new(mid_full_pos));
    market_key.store(&market_state);

    // Every order must share one inner bitmap, and the best bid and best ask
    // must be adjacent ticks (one row).
    let outer_pos = outer_pos_of(mid_full_pos);
    let outer_bitmap_index = outer_bitmap_index_of(mid_full_pos);
    for tick in BID_TICKS.iter().chain(ASK_TICKS.iter()) {
        let full_pos = full_pos_of(*tick);
        assert_eq!(outer_bitmap_index_of(full_pos), outer_bitmap_index);
        assert_eq!(outer_pos_of(full_pos), outer_pos);
    }
    assert_eq!(
        inner_pos_of(full_pos_of(ASK_TICKS[0])),
        inner_pos_of(full_pos_of(BID_TICKS[2])) + 8
    );

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    global_args.to_writer(writer, ()).unwrap();
    market_header.to_writer(writer, ()).unwrap();

    // One outer bitmap holding one inner bitmap with six updates.
    write_makes(writer, &orders);

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    set_sender();
    mock_wbtc_usdc();

    assert!(matches!(entrypoint(), Ok(())));

    // Every order rests at its tick, on the expected side of the mid.
    for tick in BID_TICKS.iter().chain(ASK_TICKS.iter()) {
        let resting_order = RestingOrderPreimage::<Pair<CustomERC20, CustomERC20>> {
            market_key,
            position: FullPos::new(full_pos_of(*tick)),
        }
        .hash()
        .load();

        assert!(resting_order.base_lots == BaseLots::new(ORDER_LOTS));
    }

    let atoms_per_lot_pair = market_header.locator.atoms_per_lot_pair();
    let base_atoms_per_lot = Base::get(&atoms_per_lot_pair);
    let quote_atoms_per_lot = Quote::get(&atoms_per_lot_pair);

    // WBTC store: the three asks each lock one lot of base.
    let base_store = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: WBTC_TOKEN,
    }
    .hash()
    .load();

    let base_locked = UnsidedLots::new(ASK_TICKS.len() as u64 * ORDER_LOTS) * base_atoms_per_lot;
    let base_deposit = UnsidedLots::new(BASE_DEPOSIT_LOTS).abs() * base_atoms_per_lot;

    assert_eq!(base_store.atoms_free, base_deposit - base_locked);
    assert_eq!(base_store.atoms_locked, base_locked);
    assert_eq!(base_store.decimals, WBTC_DECIMALS);

    // USDC store: each bid locks `price * size` quote lots, so nearer (higher)
    // bids lock more.
    let quote_store = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: USDC_TOKEN,
    }
    .hash()
    .load();

    let quote_locked_lots: u64 = BID_TICKS.iter().map(|&tick| bid_quote_lots(tick)).sum();
    let quote_locked = UnsidedLots::new(quote_locked_lots) * quote_atoms_per_lot;
    let quote_deposit = UnsidedLots::new(QUOTE_DEPOSIT_LOTS).abs() * quote_atoms_per_lot;

    assert_eq!(quote_store.atoms_free, quote_deposit - quote_locked);
    assert_eq!(quote_store.atoms_locked, quote_locked);
    assert_eq!(quote_store.decimals, USDC_DECIMALS);

    // Sanity-check the calibration: the tightest ask is 80_000 USD/BTC.
    let quote_lots_per_base_unit = WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK as u64;
    let mid_price = quote_lots_per_base_unit * WBTC_USDC_MID_TICK / USDC_LOTS_PER_UNIT as u64;
    assert_eq!(mid_price, 80_000);
}
