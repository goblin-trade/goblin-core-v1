//! A single make (resting order) placed on a fresh dynamic market.
//!
//! The market setup mirrors `tests/deposit/test_deposit_custom_erc20.rs`: a
//! dynamic `Pair<CustomERC20, ETH>` market whose locator is read straight out of
//! calldata, with the custom token listed in the calldata token list.
//!
//! A make is driven by three nested headers written after the market header,
//! matching [`goblin_core::instructions::process_makes`]:
//!
//! * one `OuterBitmapHeader` per traversed outer bitmap,
//! * one `InnerBitmapHeader` per occupied outer position inside it,
//! * one `MakeHeader` per update inside that inner bitmap.

use goblin_core::{
    axis::{
        leg::{Base, Pair},
        occupancy::OccupancyEnum,
        token::{CustomERC20, ETH},
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::INPUT_SIZE,
    market::{InnerBitmapHeader, MakeHeader, OuterBitmapHeader},
    quantities::{
        BaseLots, FullPos, InnerPos, IntoAbs, OuterBitmapIndexU32, OuterPos, UnsidedLots,
    },
    state::{Preimage, RestingOrderPreimage, StorePreimage},
    types::StoreReader,
};
use goblin_hostio::hostio_unsafe::set_test_args;

use crate::test_utils::{
    CUSTOM_TOKEN, CUSTOM_TOKEN_DECIMALS, MSG_SENDER, custom_erc20_eth_global_args,
    custom_erc20_eth_market_header, custom_erc20_list_inner, custom_market_token_data_triple,
    full_pos_of, inner_pos_of, isolated, mock_custom_token, outer_bitmap_index_of, outer_pos_of,
    set_sender,
};

/// Base lots in the single resting order.
const ORDER_LOTS: u64 = 10;

/// Base lots deposited up front, enough to collateralise the order.
const DEPOSIT_LOTS: i64 = 30;

/// Price of the resting order, in ticks.
const TICK: u64 = 100;

#[test]
fn test_make_single_order() {
    let _guard = isolated();

    let custom_tokens = custom_erc20_list_inner();
    let token_data_triple = custom_market_token_data_triple(&custom_tokens);

    let global_args = custom_erc20_eth_global_args(token_data_triple);
    let market_header = custom_erc20_eth_market_header(DEPOSIT_LOTS, 1);

    // The market key is derived in-contract from the token addresses, never read
    // from the wire, so the test derives it the same way to look the order up
    // afterwards.
    let market_key = market_header
        .locator
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    // Decompose the target tick into the `(outer bitmap index, outer pos, inner
    // pos)` triple the bitmap headers carry.
    let full_pos = full_pos_of(TICK);
    let inner_pos = inner_pos_of(full_pos);
    let outer_pos = outer_pos_of(full_pos);
    let outer_bitmap_index = outer_bitmap_index_of(full_pos);

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    // 1. Global args, then the market header (locator + local deposits).
    global_args.to_writer(writer, ()).unwrap();
    market_header.to_writer(writer, ()).unwrap();

    // 2. The make payload: one outer bitmap, one inner bitmap, one update.
    OuterBitmapHeader {
        outer_bitmap_index_u32: OuterBitmapIndexU32::new(outer_bitmap_index),
        inner_bitmap_count: 1,
    }
    .to_writer(writer, ())
    .unwrap();

    InnerBitmapHeader {
        outer_pos: OuterPos::new(outer_pos),
        update_count: 1,
    }
    .to_writer(writer, ())
    .unwrap();

    MakeHeader {
        inner_pos: InnerPos::new(inner_pos),
        occupancy_enum: OccupancyEnum::Vacant,
        // On a fresh market every position lies in the quote region, so the
        // taker leg is quote and the maker therefore locks base.
        inner_enum_raw: true,
        base_lots_u32: BaseLots::new(ORDER_LOTS as u32),
    }
    .to_writer(writer, ())
    .unwrap();

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    // 3. Set msg_sender and mock the custom token's hostio reads.
    set_sender();
    mock_custom_token();

    assert!(matches!(entrypoint(), Ok(())));

    // The single make became a stored resting order at the requested position.
    let resting_order = RestingOrderPreimage::<Pair<CustomERC20, ETH>> {
        market_key,
        position: FullPos::new(full_pos),
    }
    .hash()
    .load();

    // `BaseLots` is a sided quantity and does not implement `Debug`, so compare
    // the field directly rather than with `assert_eq!`.
    assert!(resting_order.base_lots == BaseLots::new(ORDER_LOTS));

    // It locked exactly the order size in the maker's base store, leaving the
    // rest of the deposit free.
    let store = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: CUSTOM_TOKEN,
    }
    .hash()
    .load();

    let base_atoms_per_lot = Base::get(&market_header.locator.atoms_per_lot_pair());
    let deposit_atoms = UnsidedLots::new(DEPOSIT_LOTS).abs() * base_atoms_per_lot;
    let order_atoms = UnsidedLots::new(ORDER_LOTS) * base_atoms_per_lot;

    assert_eq!(store.atoms_free, deposit_atoms - order_atoms);
    assert_eq!(store.atoms_locked, order_atoms);
    assert_eq!(store.decimals, CUSTOM_TOKEN_DECIMALS);
}
