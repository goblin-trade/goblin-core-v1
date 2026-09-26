//! Multiple resting orders placed on the same `InnerBitmap`.
//!
//! An inner bitmap is identified by an `(outer bitmap index, outer pos)` pair,
//! so two orders share it when they land in the same outer position of the same
//! outer bitmap. A single `InnerBitmapHeader` then carries `update_count = 2`,
//! followed by one `MakeHeader` per order. Each make writes a distinct
//! `inner_pos`, which is what makes the two orders occupy different positions
//! inside the shared bitmap.
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
    isolated, mock_custom_token, set_sender,
};

/// Base lots in each resting order.
const ORDER_LOTS: u64 = 10;

/// Base lots deposited up front, enough to collateralise both orders.
const DEPOSIT_LOTS: i64 = 30;

/// Ticks of the two orders. Both lie in outer position `3` of outer bitmap `0`
/// (a full position is `tick << 3`), so they share one inner bitmap while
/// sitting at different inner positions (`0x20` and `0x28`).
const TICK_0: u64 = 100;
const TICK_1: u64 = 101;

#[test]
fn test_make_multiple_orders() {
    let _guard = isolated();

    let custom_tokens = custom_erc20_list_inner();
    let token_data_triple = custom_market_token_data_triple(&custom_tokens);

    let global_args = custom_erc20_eth_global_args(token_data_triple);
    let market_header = custom_erc20_eth_market_header(DEPOSIT_LOTS, 1);

    let market_key = market_header
        .locator
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    // Both orders share the outer bitmap and outer position, so a single inner
    // bitmap is traversed with two updates.
    let full_pos_0 = TICK_0 << 3;
    let full_pos_1 = TICK_1 << 3;

    let inner_pos_0 = (full_pos_0 & 0xFF) as u8;
    let inner_pos_1 = (full_pos_1 & 0xFF) as u8;
    let outer_pos_0 = ((full_pos_0 >> 8) & 0xFF) as u8;
    let outer_pos_1 = ((full_pos_1 >> 8) & 0xFF) as u8;
    let outer_bitmap_index = (full_pos_0 >> 16) as u32;

    // The layout above only works if the two ticks really do share one inner
    // bitmap; assert that up front so the test cannot silently drift.
    assert_eq!(outer_pos_0, outer_pos_1);
    assert_eq!((full_pos_1 >> 16) as u32, outer_bitmap_index);
    assert_ne!(inner_pos_0, inner_pos_1);

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    global_args.to_writer(writer, ()).unwrap();
    market_header.to_writer(writer, ()).unwrap();

    // One outer bitmap holding one inner bitmap, which carries both updates.
    OuterBitmapHeader {
        outer_bitmap_index_u32: OuterBitmapIndexU32::new(outer_bitmap_index),
        inner_bitmap_count: 1,
    }
    .to_writer(writer, ())
    .unwrap();

    InnerBitmapHeader {
        outer_pos: OuterPos::new(outer_pos_0),
        update_count: 2,
    }
    .to_writer(writer, ())
    .unwrap();

    MakeHeader {
        inner_pos: InnerPos::new(inner_pos_0),
        occupancy_enum: OccupancyEnum::Vacant,
        inner_enum_raw: true,
        base_lots_u32: BaseLots::new(ORDER_LOTS as u32),
    }
    .to_writer(writer, ())
    .unwrap();

    MakeHeader {
        inner_pos: InnerPos::new(inner_pos_1),
        occupancy_enum: OccupancyEnum::Vacant,
        inner_enum_raw: true,
        base_lots_u32: BaseLots::new(ORDER_LOTS as u32),
    }
    .to_writer(writer, ())
    .unwrap();

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    set_sender();
    mock_custom_token();

    assert!(matches!(entrypoint(), Ok(())));

    // Both makes became stored resting orders at their requested positions.
    for full_pos in [full_pos_0, full_pos_1] {
        let resting_order = RestingOrderPreimage::<Pair<CustomERC20, ETH>> {
            market_key,
            position: FullPos::new(full_pos),
        }
        .hash()
        .load();

        assert!(resting_order.base_lots == BaseLots::new(ORDER_LOTS));
    }

    // The two orders together locked twice the order size in the maker's base
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
