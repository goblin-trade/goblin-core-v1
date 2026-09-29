//! Makes on a market that has never been seeded.
//!
//! On a fresh market `last_positions = (0, 0)`. Before `MakeRegion::Unseeded`
//! that zero quote bound made `Quote::in_region` true for every tick, so every
//! make was classified as a quote-region order and bids (which must be `In =
//! Base`) were rejected with `InvalidOpenPrice`. The first make now seats the
//! opener on its own side and pushes the opposite bound to the far edge of the
//! price range (`MAX` for the quote leg, `MIN` for the base leg), so:
//!
//! * a lone bid rests on a fresh market and its own tick stays on the base side,
//! * a following ask can rest on the other side of the freshly opened market,
//! * a second bid at the opening tick is still accepted.

use goblin_core::{
    axis::{
        leg::{Base, Pair, Quote},
        token::CustomERC20,
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::INPUT_SIZE,
    quantities::{BaseLots, FullPos, FullPosition},
    state::{MarketPreimage, Preimage, RestingOrderPreimage, SlotKey},
    types::StoreReader,
};
use goblin_hostio::hostio_unsafe::set_test_args;

use crate::test_utils::{
    MakeOrder, WBTC_USDC_MID_TICK, full_pos_of, isolated, make_outer_bitmap_count, mock_wbtc_usdc,
    outer_bitmap_index_of, set_sender, wbtc_usdc_global_args, wbtc_usdc_list_inner,
    wbtc_usdc_market_header, wbtc_usdc_token_data_triple, write_makes,
};

/// Base lots in each order.
const ORDER_LOTS: u64 = 1;

/// WBTC deposited up front: enough to collateralise asks.
const BASE_DEPOSIT_LOTS: i64 = 10;

/// USDC deposited up front: enough to collateralise bids.
const QUOTE_DEPOSIT_LOTS: i64 = 300_000;

/// The dynamic WBTC/USDC market state key.
type MarketKey = SlotKey<MarketPreimage<Pair<CustomERC20, CustomERC20>>>;

/// Process one make call carrying `orders` on an otherwise untouched market and
/// return the market key so callers can inspect what was stored.
fn run_fresh_make(orders: &[MakeOrder]) -> MarketKey {
    let tokens = wbtc_usdc_list_inner();
    let token_data_triple = wbtc_usdc_token_data_triple(&tokens);

    let global_args = wbtc_usdc_global_args(token_data_triple);
    let market_header = wbtc_usdc_market_header(
        BASE_DEPOSIT_LOTS,
        QUOTE_DEPOSIT_LOTS,
        make_outer_bitmap_count(orders),
    );

    let market_key = market_header
        .locator
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    global_args.to_writer(writer, ()).unwrap();
    market_header.to_writer(writer, ()).unwrap();
    write_makes(writer, orders);

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    set_sender();
    mock_wbtc_usdc();

    assert!(matches!(entrypoint(), Ok(())));
    market_key
}

/// Assert the resting order at `position` holds `ORDER_LOTS`.
fn assert_rests_at(market_key: MarketKey, position: FullPos) {
    let resting_order = RestingOrderPreimage::<Pair<CustomERC20, CustomERC20>> {
        market_key,
        position,
    }
    .hash()
    .load();

    assert!(resting_order.base_lots == BaseLots::new(ORDER_LOTS));
}

#[test]
fn test_make_bid_on_fresh_market() {
    let _guard = isolated();

    // A single bid (In = Base) below the mid. This is the case that used to be
    // rejected with `InvalidOpenPrice`.
    let tick = WBTC_USDC_MID_TICK - 3;
    let market_key = run_fresh_make(&[MakeOrder {
        tick,
        base_lots: ORDER_LOTS as u32,
        inner_enum_raw: false,
    }]);

    assert_rests_at(market_key, FullPos::new(full_pos_of(tick)));

    // The bid owns the base bound; the untouched quote bound is pushed to MAX so
    // the (empty) spread above the bid accepts asks.
    let market_state = market_key.load();
    assert!(Base::get(&market_state.last_positions) == FullPos::new(full_pos_of(tick)));
    assert!(Quote::get(&market_state.last_positions) == FullPos::MAX);
}

#[test]
fn test_make_fresh_market_both_sides() {
    let _guard = isolated();

    // Bid first: it is the order that has to open the unseeded market.
    let bid_tick = WBTC_USDC_MID_TICK - 3;
    let ask_tick = WBTC_USDC_MID_TICK + 3;

    // Both ticks must share one outer bitmap so a single make call can carry
    // both sides.
    assert_eq!(
        outer_bitmap_index_of(full_pos_of(bid_tick)),
        outer_bitmap_index_of(full_pos_of(ask_tick))
    );

    let market_key = run_fresh_make(&[
        MakeOrder {
            tick: bid_tick,
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick: ask_tick,
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: true,
        },
    ]);

    // Both sides rest: the bid below the opened mid, the ask trading through the
    // spread above it.
    assert_rests_at(market_key, FullPos::new(full_pos_of(bid_tick)));
    assert_rests_at(market_key, FullPos::new(full_pos_of(ask_tick)));

    // The ask tightened the quote bound from MAX down to its own tick; the base
    // bound is still the opening bid.
    let market_state = market_key.load();
    assert!(Base::get(&market_state.last_positions) == FullPos::new(full_pos_of(bid_tick)));
    assert!(Quote::get(&market_state.last_positions) == FullPos::new(full_pos_of(ask_tick)));
}

#[test]
fn test_make_second_bid_at_opening_tick() {
    let _guard = isolated();

    // The opening bid leaves the quote bound at MAX, so the bid's own tick stays
    // on the base side and can be used again. With the earlier both-legs-at-`T`
    // seed, this second bid would have been classified `In(Quote)` and rejected.
    let tick = WBTC_USDC_MID_TICK - 3;
    let market_key = run_fresh_make(&[
        MakeOrder {
            tick,
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: false,
        },
        MakeOrder {
            tick,
            base_lots: ORDER_LOTS as u32,
            inner_enum_raw: false,
        },
    ]);

    // A repeated tick is stacked into the next inner column, so the second order
    // sits one column above the first. Both rest.
    let first = FullPos::new(full_pos_of(tick));
    let second = FullPos::new(full_pos_of(tick) + 1);
    assert_rests_at(market_key, first);
    assert_rests_at(market_key, second);

    // The quote bound is untouched; the base bound tracks the highest base order.
    let market_state = market_key.load();
    assert!(Base::get(&market_state.last_positions) == second);
    assert!(Quote::get(&market_state.last_positions) == FullPos::MAX);
}
