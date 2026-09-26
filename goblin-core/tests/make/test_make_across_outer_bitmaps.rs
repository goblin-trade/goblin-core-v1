//! Multiple resting orders spread across two outer bitmap indices.
//!
//! An outer bitmap is identified by the high bits of the position: a full
//! position is `tick << 3` and its outer bitmap index is `full_pos >> 16`, so
//! one outer bitmap spans `1 << 13` consecutive ticks. Two orders therefore hit
//! different outer bitmaps when their ticks differ by `1 << 13`.
//!
//! The payload carries `outer_bitmap_count = 2`, and each outer bitmap holds its
//! own inner bitmap with a single make, matching
//! [`goblin_core::instructions::process_makes`].
//!
//! The market setup mirrors `test_make_single_order.rs`: a dynamic
//! `Pair<CustomERC20, ETH>` market whose locator is read straight out of
//! calldata.

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

/// Base lots in each resting order.
const ORDER_LOTS: u64 = 10;

/// Base lots deposited up front, enough to collateralise both orders.
const DEPOSIT_LOTS: i64 = 30;

/// Ticks of the two orders. The second is one whole outer bitmap (`1 << 13`
/// ticks) above the first, so the two share the same inner and outer position
/// but live in different outer bitmap indices.
const TICK_0: u64 = 100;
const TICK_1: u64 = TICK_0 + (1 << 13);

#[test]
fn test_make_across_outer_bitmaps() {
    let _guard = isolated();

    let custom_tokens = custom_erc20_list_inner();
    let token_data_triple = custom_market_token_data_triple(&custom_tokens);

    let global_args = custom_erc20_eth_global_args(token_data_triple);
    let market_header = custom_erc20_eth_market_header(DEPOSIT_LOTS, 2);

    let market_key = market_header
        .locator
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    let full_pos_0 = full_pos_of(TICK_0);
    let full_pos_1 = full_pos_of(TICK_1);

    let outer_bitmap_index_0 = outer_bitmap_index_of(full_pos_0);
    let outer_bitmap_index_1 = outer_bitmap_index_of(full_pos_1);

    // The two ticks differ by exactly one outer bitmap, and nothing else: same
    // inner and outer position, adjacent outer bitmap indices.
    assert_eq!(outer_bitmap_index_1, outer_bitmap_index_0 + 1);
    assert_eq!(inner_pos_of(full_pos_0), inner_pos_of(full_pos_1));
    assert_eq!(outer_pos_of(full_pos_0), outer_pos_of(full_pos_1));

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    global_args.to_writer(writer, ()).unwrap();
    market_header.to_writer(writer, ()).unwrap();

    // Two outer bitmaps, each holding one inner bitmap with a single update.
    for full_pos in [full_pos_0, full_pos_1] {
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
            // On a fresh market every position lies in the quote region, so the
            // taker leg is quote and the maker therefore locks base.
            inner_enum_raw: true,
            base_lots_u32: BaseLots::new(ORDER_LOTS as u32),
        }
        .to_writer(writer, ())
        .unwrap();
    }

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    set_sender();
    mock_custom_token();

    assert!(matches!(entrypoint(), Ok(())));

    // Each make became a stored resting order at its requested position.
    for full_pos in [full_pos_0, full_pos_1] {
        let resting_order = RestingOrderPreimage::<Pair<CustomERC20, ETH>> {
            market_key,
            position: FullPos::new(full_pos),
        }
        .hash()
        .load();

        assert!(resting_order.base_lots == BaseLots::new(ORDER_LOTS));
    }

    // Both orders together locked twice the order size in the maker's base
    // store, leaving the rest of the deposit free.
    let store = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: CUSTOM_TOKEN,
    }
    .hash()
    .load();

    let base_atoms_per_lot = Base::get(&market_header.locator.atoms_per_lot_pair());
    let deposit_atoms = UnsidedLots::new(DEPOSIT_LOTS).abs() * base_atoms_per_lot;
    let orders_atoms = UnsidedLots::new(ORDER_LOTS * 2) * base_atoms_per_lot;

    assert_eq!(store.atoms_free, deposit_atoms - orders_atoms);
    assert_eq!(store.atoms_locked, orders_atoms);
    assert_eq!(store.decimals, CUSTOM_TOKEN_DECIMALS);
}
