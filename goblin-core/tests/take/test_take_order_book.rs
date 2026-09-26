//! A realistic two-sided WBTC/USDC book, made in one call and swept in another.
//!
//! The book is exactly the one built by `tests/make/test_make_two_sided_book.rs`:
//! three bids below the 80_000 USD/BTC mid and three asks at/above it, all inside
//! one inner bitmap. The maker builds it in a first `entrypoint()` call; a
//! different trader then sweeps the whole ask side in a second call, so the
//! match consumes the three price levels best-first.
//!
//! Making and taking are separate smart-contract calls, and the make phase is
//! reused from the make tests via `test_utils::write_makes`.

use goblin_core::{
    axis::{
        leg::{Base, Pair, Quote},
        token::CustomERC20,
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::INPUT_SIZE,
    instructions::{TakeFlags, TakeHeader},
    quantities::{FullPos, FullPosU32, QuoteLots, UnsidedLots},
    state::{Preimage, StorePreimage},
    types::StoreReader,
};
use goblin_hostio::hostio_unsafe::set_test_args;

use crate::test_utils::{
    MSG_SENDER, MakeOrder, TAKER, USDC_DECIMALS, USDC_LOTS_PER_UNIT, USDC_TOKEN, WBTC_DECIMALS,
    WBTC_LOTS_PER_UNIT, WBTC_TOKEN, WBTC_USDC_MID_TICK,
    WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK, full_pos_of, isolated, make_outer_bitmap_count,
    mock_wbtc_usdc, reset_call_state, set_sender, set_taker, wbtc_usdc_global_args,
    wbtc_usdc_list_inner, wbtc_usdc_market, wbtc_usdc_market_header, wbtc_usdc_take_market_header,
    wbtc_usdc_token_data_triple, write_makes,
};

/// Base lots in each resting order: `0.01` WBTC.
const ORDER_LOTS: u32 = 1;

/// WBTC the maker deposits up front: enough to collateralise the three asks.
const BASE_DEPOSIT_LOTS: i64 = 10;

/// USDC the maker deposits up front: enough to collateralise the three bids.
const QUOTE_DEPOSIT_LOTS: i64 = 300_000;

/// Bid ticks, below the mid. Best bid last.
const BID_TICKS: [u64; 3] = [
    WBTC_USDC_MID_TICK - 3,
    WBTC_USDC_MID_TICK - 2,
    WBTC_USDC_MID_TICK - 1,
];

/// Ask ticks, at/above the mid. Best ask first.
const ASK_TICKS: [u64; 3] = [
    WBTC_USDC_MID_TICK,
    WBTC_USDC_MID_TICK + 1,
    WBTC_USDC_MID_TICK + 2,
];

/// USDC the taker deposits up front, a little more than it spends.
const TAKER_QUOTE_DEPOSIT_LOTS: i64 = 300_000;

/// Quote lots a maker locks (bid) or a taker spends (filling an ask) for
/// `base_lots` at `tick`.
fn quote_lots_at(tick: u64, base_lots: u32) -> u64 {
    WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK as u64 * tick * base_lots as u64
        / WBTC_LOTS_PER_UNIT as u64
}

#[test]
fn test_take_order_book() {
    let _guard = isolated();

    let tokens = wbtc_usdc_list_inner();
    let token_data_triple = wbtc_usdc_token_data_triple(&tokens);
    let market_key = wbtc_usdc_market()
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    let bid_locked_lots: u64 = BID_TICKS
        .iter()
        .map(|&tick| quote_lots_at(tick, ORDER_LOTS))
        .sum();
    let ask_take_lots: u64 = ASK_TICKS
        .iter()
        .map(|&tick| quote_lots_at(tick, ORDER_LOTS))
        .sum();
    let taker_quote_lots: u32 = ask_take_lots as u32;

    // --- Call 1: the maker builds the two-sided book ----------------------
    // `write_makes` groups consecutive orders by inner bitmap, so the bids and
    // asks below share one inner bitmap (all at outer pos 56 of bitmap 1).
    let orders = [
        MakeOrder {
            tick: BID_TICKS[0],
            base_lots: ORDER_LOTS,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick: BID_TICKS[1],
            base_lots: ORDER_LOTS,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick: BID_TICKS[2],
            base_lots: ORDER_LOTS,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick: ASK_TICKS[0],
            base_lots: ORDER_LOTS,
            inner_enum_raw: true,
        },
        MakeOrder {
            tick: ASK_TICKS[1],
            base_lots: ORDER_LOTS,
            inner_enum_raw: true,
        },
        MakeOrder {
            tick: ASK_TICKS[2],
            base_lots: ORDER_LOTS,
            inner_enum_raw: true,
        },
    ];

    // Seed the last positions to the mid so bids (below mid) are allowed.
    let mid_full_pos = full_pos_of(WBTC_USDC_MID_TICK);
    let mut market_state = market_key.load();
    market_state.last_positions = Pair::new(FullPos::new(mid_full_pos), FullPos::new(mid_full_pos));
    market_key.store(&market_state);

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    wbtc_usdc_global_args(token_data_triple)
        .to_writer(writer, ())
        .unwrap();
    wbtc_usdc_market_header(
        BASE_DEPOSIT_LOTS,
        QUOTE_DEPOSIT_LOTS,
        make_outer_bitmap_count(&orders),
    )
    .to_writer(writer, ())
    .unwrap();
    write_makes(writer, &orders);

    set_test_args(writer.get_calldata().to_vec());
    set_sender();
    mock_wbtc_usdc();

    let make_result = entrypoint();
    assert!(matches!(make_result, Ok(())), "make: {make_result:?}");

    // --- Call 2: a different trader sweeps the ask side -------------------
    reset_call_state();

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    wbtc_usdc_global_args(token_data_triple)
        .to_writer(writer, ())
        .unwrap();
    // Only the quote leg (taker buys base) runs, so only the asks are matched.
    wbtc_usdc_take_market_header(0, TAKER_QUOTE_DEPOSIT_LOTS, Pair::new(false, true))
        .to_writer(writer, ())
        .unwrap();

    TakeHeader::<Quote> {
        flags: TakeFlags {
            read_min_lots: false,
            read_limit: false,
        },
        // Spend exactly enough to clear the three ask levels.
        num_lots_u32: QuoteLots::new(taker_quote_lots),
        min_lots_to_fill_u32: QuoteLots::new(0),
        limit_u32: FullPosU32::default(),
    }
    .to_writer(writer, ())
    .unwrap();

    set_test_args(writer.get_calldata().to_vec());
    set_taker();
    mock_wbtc_usdc();

    let take_result = entrypoint();
    assert!(matches!(take_result, Ok(())), "take: {take_result:?}");

    let atoms_per_lot_pair = wbtc_usdc_market().atoms_per_lot_pair();
    let base_atoms_per_lot = Base::get(&atoms_per_lot_pair);
    let quote_atoms_per_lot = Quote::get(&atoms_per_lot_pair);

    let maker_base = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: WBTC_TOKEN,
    }
    .hash()
    .load();
    let maker_quote = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: USDC_TOKEN,
    }
    .hash()
    .load();
    let taker_base = StorePreimage::<CustomERC20> {
        trader: TAKER,
        token_address: WBTC_TOKEN,
    }
    .hash()
    .load();
    let taker_quote = StorePreimage::<CustomERC20> {
        trader: TAKER,
        token_address: USDC_TOKEN,
    }
    .hash()
    .load();

    let asks_filled_lots = ASK_TICKS.len() as u64 * ORDER_LOTS as u64;

    // Maker: the three asks are released (base locked drops to zero) and the
    // proceeds are credited, while the bids stay locked and untouched.
    let maker_base_free =
        UnsidedLots::new((BASE_DEPOSIT_LOTS - asks_filled_lots as i64) as u64) * base_atoms_per_lot;
    assert_eq!(maker_base.atoms_free, maker_base_free);
    assert_eq!(
        maker_base.atoms_locked,
        UnsidedLots::new(0u64) * base_atoms_per_lot
    );
    assert_eq!(maker_base.decimals, WBTC_DECIMALS);

    let maker_quote_free_lots = (QUOTE_DEPOSIT_LOTS as u64 - bid_locked_lots) + ask_take_lots;
    assert_eq!(
        maker_quote.atoms_free,
        UnsidedLots::new(maker_quote_free_lots) * quote_atoms_per_lot
    );
    assert_eq!(
        maker_quote.atoms_locked,
        UnsidedLots::new(bid_locked_lots) * quote_atoms_per_lot
    );
    assert_eq!(maker_quote.decimals, USDC_DECIMALS);

    // Taker: bought base across the three levels, paid quote.
    let taker_base_free = UnsidedLots::new(asks_filled_lots) * base_atoms_per_lot;
    let taker_quote_free =
        UnsidedLots::new((TAKER_QUOTE_DEPOSIT_LOTS - ask_take_lots as i64) as u64)
            * quote_atoms_per_lot;
    assert_eq!(taker_base.atoms_free, taker_base_free);
    assert_eq!(
        taker_base.atoms_locked,
        UnsidedLots::new(0u64) * base_atoms_per_lot
    );
    assert_eq!(taker_base.decimals, WBTC_DECIMALS);
    assert_eq!(taker_quote.atoms_free, taker_quote_free);
    assert_eq!(
        taker_quote.atoms_locked,
        UnsidedLots::new(0u64) * quote_atoms_per_lot
    );
    assert_eq!(taker_quote.decimals, USDC_DECIMALS);

    // Sanity-check the calibration used above.
    assert_eq!(
        WBTC_USDC_QUOTE_LOTS_PER_BASE_UNIT_PER_TICK as u64 * WBTC_USDC_MID_TICK
            / USDC_LOTS_PER_UNIT as u64,
        80_000
    );
}
