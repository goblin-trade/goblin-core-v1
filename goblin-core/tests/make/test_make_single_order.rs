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
        market::Dynamic,
        occupancy::OccupancyEnum,
        token::{
            CustomERC20, CustomERC20Stub, ETH, ETHStub, TokenDataTriple,
            token_list::CustomERC20List,
            token_marker::{CustomERC20Index, TokenData},
        },
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, INPUT_SIZE, MarketCounts, MarketCountsInner,
    },
    market::{CommonMarket, InnerBitmapHeader, MakeHeader, MarketHeader, OuterBitmapHeader},
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, FullPos, InnerPos, IntoAbs, OuterBitmapIndexU32, OuterPos,
        QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, UnsidedAtoms, UnsidedLots,
    },
    settlement::LocalDeposits,
    state::{Preimage, RestingOrderPreimage, StorePreimage},
    types::{SameTriple, StoreReader},
};
use goblin_hostio::hostio_unsafe::set_test_args;

use crate::test_utils::{
    CUSTOM_TOKEN, CUSTOM_TOKEN_DECIMALS, MSG_SENDER, mock_decimals, mock_transfer_from, set_sender,
};

/// Base lots in the single resting order.
const ORDER_LOTS: u64 = 10;

/// Base lots deposited up front, enough to collateralise the order.
const DEPOSIT_LOTS: i64 = 30;

/// Price of the resting order, in ticks.
const TICK: u64 = 100;

#[test]
fn test_make_single_order() {
    // A custom ERC20 can never live in a hardcoded market, so it is listed in
    // calldata and traded through a dynamic market.
    let custom_token_data = TokenData::<CustomERC20> {
        address: CUSTOM_TOKEN,
        decimals: CustomERC20Stub,
    };
    let custom_erc20_list_inner: [TokenData<CustomERC20>; 1] = [custom_token_data];
    let custom_erc20_count = custom_erc20_list_inner.len();
    let custom_erc20_list = CustomERC20List {
        inner: custom_erc20_list_inner.as_ref(),
    };
    let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

    // One dynamic market for `Pair<CustomERC20, ETH>`. Legs are indexed
    // `[market][base][quote]`, each ordered by variant:
    // markets `[Hardcoded, Dynamic]`, tokens `[ETH, HardcodedERC20, CustomERC20]`.
    let market_counts = MarketCounts::new(
        MarketCountsInner::default(), // hardcoded markets
        MarketCountsInner::new(
            SameTriple::new(0, 0, 0), // base ETH
            SameTriple::new(0, 0, 0), // base HardcodedERC20
            SameTriple::new(1, 0, 0), // base CustomERC20, quote ETH
        ),
    );

    let global_args = GlobalArgs {
        flags: HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: true,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count,
        },
        header: Header {
            eth_out_due_u32: UnsidedAtoms::default(),
            market_counts,
        },
        refs: HeaderRefs {
            custom_recipient: None,
            token_data_triple,
        },
    };

    let common_market = CommonMarket::<Pair<CustomERC20, ETH>>::new(
        Pair::new(CustomERC20Index::from(0), ETHStub),
        Pair::new(
            BaseLotsPerBaseUnit::new(100),
            QuoteLotsPerQuoteUnit::new(100),
        ),
        QuoteLotsPerBaseUnitPerTick::new(1),
    );

    // The resting order is keyed by `(market, position)`. The market key is
    // derived in-contract from the token addresses, never read from the wire, so
    // the test derives it the same way to look the order up afterwards.
    let market_key = common_market
        .get_preimage(&token_data_triple)
        .unwrap()
        .hash();

    // The make locks base, so fund it with a local base deposit.
    let deposit_lots_delta = UnsidedLots::new(DEPOSIT_LOTS);
    let local_deposits = LocalDeposits::<Pair<CustomERC20, ETH>>::new(deposit_lots_delta, ETHStub);

    let market_header = MarketHeader::<(Dynamic, Pair<CustomERC20, ETH>)> {
        decode_deposit_amounts: true,
        execute_takes: Pair::new(false, false),
        outer_bitmap_count: 1,
        local_deposits,
        locator: common_market,
    };

    // Decompose the target tick into the `(outer bitmap index, outer pos, inner
    // pos)` triple the bitmap headers carry. A full position is `tick << 3`,
    // split into 3 bits of column, 5 of row, 8 of outer pos and 48 of index.
    let full_pos = TICK << 3;
    let inner_pos = (full_pos & 0xFF) as u8;
    let outer_pos = ((full_pos >> 8) & 0xFF) as u8;
    let outer_bitmap_index = (full_pos >> 16) as u32;

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

    // 3. Set msg_sender. The store is created here, so its decimals are read via
    //    hostio and the funding deposit is pulled with `transferFrom`.
    set_sender();
    mock_decimals(custom_token_data.address, CUSTOM_TOKEN_DECIMALS);
    mock_transfer_from(custom_token_data.address);

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
        token_address: custom_token_data.address,
    }
    .hash()
    .load();

    let base_atoms_per_lot = Base::get(&market_header.locator.atoms_per_lot_pair());
    let deposit_atoms = deposit_lots_delta.abs() * base_atoms_per_lot;
    let order_atoms = UnsidedLots::new(ORDER_LOTS) * base_atoms_per_lot;

    assert_eq!(store.atoms_free, deposit_atoms - order_atoms);
    assert_eq!(store.atoms_locked, order_atoms);
    assert_eq!(store.decimals, CUSTOM_TOKEN_DECIMALS);
}
