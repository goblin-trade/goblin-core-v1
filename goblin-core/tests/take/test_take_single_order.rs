//! A single resting order, made and then taken in a separate call.
//!
//! The maker rests an ask (sells `0.05` WBTC at the 80_000 USD/BTC mid) in the
//! make call. A different trader then takes part of it in a second
//! `entrypoint()` call, paying USDC and receiving WBTC.
//!
//! Making and taking are separate smart-contract calls: the resting order and
//! balances live in storage between the two `entrypoint()` runs.
//!
//! Because the taker only fills part of the order, the unfilled remainder is
//! written back and the taker-side match is bounded by its budget.

use goblin_core::{
    axis::{
        leg::{Base, Pair, Quote},
        token::CustomERC20,
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::INPUT_SIZE,
    instructions::{TakeFlags, TakeHeader},
    quantities::{BaseLots, FullPos, FullPosU32, QuoteLots, UnsidedLots},
    state::{Preimage, RestingOrderPreimage, StorePreimage},
    types::StoreReader,
};
use goblin_hostio::hostio_unsafe::set_test_args;

use crate::test_utils::{
    MSG_SENDER, MakeOrder, TAKER, USDC_DECIMALS, USDC_TOKEN, WBTC_DECIMALS, WBTC_TOKEN,
    full_pos_of, isolated, make_outer_bitmap_count, mock_wbtc_usdc, reset_call_state, set_sender,
    set_taker, wbtc_usdc_global_args, wbtc_usdc_list_inner, wbtc_usdc_market,
    wbtc_usdc_market_header, wbtc_usdc_take_market_header, wbtc_usdc_token_data_triple,
    write_makes,
};

/// Base lots in the resting ask: `0.05` WBTC.
const ASK_LOTS: u32 = 5;

/// WBTC the maker deposits up front.
const MAKER_BASE_DEPOSIT_LOTS: i64 = 10;

/// Tick of the resting ask: the 80_000 USD/BTC mid.
const ASK_TICK: u64 = 10_000;

/// Base lots the taker fills.
const TAKE_BASE_LOTS: u64 = 2;

/// Quote lots the taker spends: `price * size / base_lots_per_unit`.
const TAKE_QUOTE_LOTS: u32 = 160_000;

/// USDC the taker deposits up front, a little more than it spends.
const TAKER_QUOTE_DEPOSIT_LOTS: i64 = 200_000;

#[test]
fn test_take_single_order() {
    let _guard = isolated();

    let tokens = wbtc_usdc_list_inner();
    let token_data_triple = wbtc_usdc_token_data_triple(&tokens);
    let market_key = wbtc_usdc_market()
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    // --- Call 1: the maker rests a single ask -----------------------------
    let orders = [MakeOrder {
        tick: ASK_TICK,
        base_lots: ASK_LOTS,
        inner_enum_raw: true,
    }];

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    wbtc_usdc_global_args(token_data_triple)
        .to_writer(writer, ())
        .unwrap();
    wbtc_usdc_market_header(MAKER_BASE_DEPOSIT_LOTS, 0, make_outer_bitmap_count(&orders))
        .to_writer(writer, ())
        .unwrap();
    write_makes(writer, &orders);

    set_test_args(writer.get_calldata().to_vec());
    set_sender();
    mock_wbtc_usdc();

    let make_result = entrypoint();
    assert!(matches!(make_result, Ok(())), "make: {make_result:?}");

    // --- Call 2: a different trader takes part of the ask -----------------
    reset_call_state();

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    wbtc_usdc_global_args(token_data_triple)
        .to_writer(writer, ())
        .unwrap();
    // `execute_takes = (Base, Quote)`; only the quote leg (taker buys base) runs.
    wbtc_usdc_take_market_header(0, TAKER_QUOTE_DEPOSIT_LOTS, Pair::new(false, true))
        .to_writer(writer, ())
        .unwrap();

    TakeHeader::<Quote> {
        flags: TakeFlags {
            read_min_lots: false,
            read_limit: false,
        },
        num_lots_u32: QuoteLots::new(TAKE_QUOTE_LOTS),
        min_lots_to_fill_u32: QuoteLots::new(0),
        limit_u32: FullPosU32::default(),
    }
    .to_writer(writer, ())
    .unwrap();

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());
    set_taker();
    mock_wbtc_usdc();

    let take_result = entrypoint();
    assert!(matches!(take_result, Ok(())), "take: {take_result:?}");

    let atoms_per_lot_pair = wbtc_usdc_market().atoms_per_lot_pair();
    let base_atoms_per_lot = Base::get(&atoms_per_lot_pair);
    let quote_atoms_per_lot = Quote::get(&atoms_per_lot_pair);

    // The unfilled remainder of the ask is written back.
    let resting_order = RestingOrderPreimage::<Pair<CustomERC20, CustomERC20>> {
        market_key,
        position: FullPos::new(full_pos_of(ASK_TICK)),
    }
    .hash()
    .load();
    assert!(resting_order.maker == MSG_SENDER);
    assert!(resting_order.base_lots == BaseLots::new((ASK_LOTS as u64) - TAKE_BASE_LOTS));

    // The maker's lock is released for the filled part and the proceeds are
    // credited to it.
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

    let maker_base_free =
        UnsidedLots::new((MAKER_BASE_DEPOSIT_LOTS - ASK_LOTS as i64) as u64) * base_atoms_per_lot;
    let maker_base_locked =
        UnsidedLots::new((ASK_LOTS as u64) - TAKE_BASE_LOTS) * base_atoms_per_lot;
    let maker_quote_free = UnsidedLots::new(TAKE_QUOTE_LOTS as u64) * quote_atoms_per_lot;

    assert_eq!(maker_base.atoms_free, maker_base_free);
    assert_eq!(maker_base.atoms_locked, maker_base_locked);
    assert_eq!(maker_base.decimals, WBTC_DECIMALS);
    assert_eq!(maker_quote.atoms_free, maker_quote_free);

    // The taker paid quote and received base.
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

    let taker_base_free = UnsidedLots::new(TAKE_BASE_LOTS) * base_atoms_per_lot;
    let taker_quote_free =
        UnsidedLots::new((TAKER_QUOTE_DEPOSIT_LOTS - TAKE_QUOTE_LOTS as i64) as u64)
            * quote_atoms_per_lot;

    assert_eq!(taker_base.atoms_free, taker_base_free);
    assert_eq!(
        taker_base.atoms_locked,
        UnsidedLots::new(0u64) * base_atoms_per_lot
    );
    assert_eq!(taker_base.decimals, WBTC_DECIMALS);
    assert_eq!(taker_quote.atoms_free, taker_quote_free);
    assert_eq!(taker_quote.decimals, USDC_DECIMALS);
}
