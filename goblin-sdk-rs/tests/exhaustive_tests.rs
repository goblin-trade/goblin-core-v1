use goblin_core_v1::{
    axis::leg::LegEnum,
    input_processor::{ArgsReader, FixedDecode, HeaderFlags, VariableDecode},
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, InnerPos, OuterBitmapIndex, OuterPos,
        QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit,
    },
};
use goblin_sdk_rs::{
    position_from_parts, GoblinCalldataBuilder, GoblinSdkError, MarketCallBuilder, MarketSpecIndex,
    TokenKind,
};

#[test]
fn test_all_11_market_specs_in_single_call() {
    let mut builder = GoblinCalldataBuilder::new();

    // Register 2 custom tokens
    let token_c0 = [0x10; 20];
    let token_c1 = [0x20; 20];
    let c0 = builder.add_custom_token(token_c0).unwrap();
    let c1 = builder.add_custom_token(token_c1).unwrap();

    let base_lot_size = BaseLotsPerBaseUnit::new(100);
    let quote_lot_size = QuoteLotsPerQuoteUnit::new(1000);
    let tick_size = QuoteLotsPerBaseUnitPerTick::new(1);

    // Add one market for each of the 11 specs
    // Spec 0: Hardcoded (ETH, HardcodedERC20)
    let mut m0 = MarketCallBuilder::new();
    m0.hardcoded(MarketSpecIndex::HardcodedEthHardcodedERC20, 0)
        .unwrap();
    builder.add_market(m0.build().unwrap());

    // Spec 1: Hardcoded (HardcodedERC20, ETH)
    let mut m1 = MarketCallBuilder::new();
    m1.hardcoded(MarketSpecIndex::HardcodedHardcodedERC20Eth, 0)
        .unwrap();
    builder.add_market(m1.build().unwrap());

    // Spec 2: Hardcoded (HardcodedERC20, HardcodedERC20)
    let mut m2 = MarketCallBuilder::new();
    m2.hardcoded(MarketSpecIndex::HardcodedHardcodedERC20HardcodedERC20, 0)
        .unwrap();
    builder.add_market(m2.build().unwrap());

    // Spec 3: Dynamic (ETH, HardcodedERC20)
    let mut m3 = MarketCallBuilder::new();
    m3.dynamic(
        TokenKind::ETH,
        0,
        TokenKind::HardcodedERC20,
        0,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m3.build().unwrap());

    // Spec 4: Dynamic (ETH, CustomERC20)
    let mut m4 = MarketCallBuilder::new();
    m4.dynamic(
        TokenKind::ETH,
        0,
        TokenKind::CustomERC20,
        c0,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m4.build().unwrap());

    // Spec 5: Dynamic (HardcodedERC20, ETH)
    let mut m5 = MarketCallBuilder::new();
    m5.dynamic(
        TokenKind::HardcodedERC20,
        1,
        TokenKind::ETH,
        0,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m5.build().unwrap());

    // Spec 6: Dynamic (HardcodedERC20, HardcodedERC20)
    let mut m6 = MarketCallBuilder::new();
    m6.dynamic(
        TokenKind::HardcodedERC20,
        0,
        TokenKind::HardcodedERC20,
        1,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m6.build().unwrap());

    // Spec 7: Dynamic (HardcodedERC20, CustomERC20)
    let mut m7 = MarketCallBuilder::new();
    m7.dynamic(
        TokenKind::HardcodedERC20,
        0,
        TokenKind::CustomERC20,
        c1,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m7.build().unwrap());

    // Spec 8: Dynamic (CustomERC20, ETH)
    let mut m8 = MarketCallBuilder::new();
    m8.dynamic(
        TokenKind::CustomERC20,
        c0,
        TokenKind::ETH,
        0,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m8.build().unwrap());

    // Spec 9: Dynamic (CustomERC20, HardcodedERC20)
    let mut m9 = MarketCallBuilder::new();
    m9.dynamic(
        TokenKind::CustomERC20,
        c1,
        TokenKind::HardcodedERC20,
        0,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m9.build().unwrap());

    // Spec 10: Dynamic (CustomERC20, CustomERC20)
    let mut m10 = MarketCallBuilder::new();
    m10.dynamic(
        TokenKind::CustomERC20,
        c0,
        TokenKind::CustomERC20,
        c1,
        base_lot_size,
        quote_lot_size,
        tick_size,
    )
    .unwrap();
    builder.add_market(m10.build().unwrap());

    let calldata = builder.build().expect("build 11-market payload");
    let hex_calldata = builder.build_hex().expect("build hex");
    assert!(hex_calldata.starts_with("0x"));

    // Verify decoding of all 11 counts
    let reader = ArgsReader::from_slice(&calldata);
    let flags = HeaderFlags::try_fixed_decode(&reader).expect("decode flags");
    assert!(flags.process_dynamic_markets);
    assert_eq!(flags.custom_erc20_count, 2);

    let global_header =
        goblin_core_v1::input_processor::GlobalHeader::try_variable_decode(&reader, &flags)
            .expect("decode global header");

    for spec_idx in 0..11 {
        let count = global_header.market_counts.get_count_and_advance();
        assert_eq!(count, 1, "Market count for spec {} must be 1", spec_idx);
    }
}

#[test]
fn test_all_make_actions_encoding() {
    let mut builder = GoblinCalldataBuilder::new();
    let mut market = MarketCallBuilder::new();
    market
        .hardcoded(MarketSpecIndex::HardcodedEthHardcodedERC20, 0)
        .unwrap()
        .make_open(
            position_from_parts(OuterBitmapIndex::new(0), OuterPos::new(0), InnerPos::new(1)),
            LegEnum::Base,
            BaseLots::new(10),
        )
        .make_open(
            position_from_parts(OuterBitmapIndex::new(0), OuterPos::new(0), InnerPos::new(2)),
            LegEnum::Quote,
            BaseLots::new(20),
        )
        .make_increase(
            position_from_parts(OuterBitmapIndex::new(0), OuterPos::new(0), InnerPos::new(3)),
            BaseLots::new(30),
        )
        .make_decrease(
            position_from_parts(OuterBitmapIndex::new(0), OuterPos::new(0), InnerPos::new(4)),
            BaseLots::new(15),
        )
        .make_close(
            position_from_parts(OuterBitmapIndex::new(0), OuterPos::new(0), InnerPos::new(5)),
            BaseLots::new(40),
        );

    builder.add_market(market.build().unwrap());
    let calldata = builder.build().unwrap();

    let reader = ArgsReader::from_slice(&calldata);
    let flags = HeaderFlags::try_fixed_decode(&reader).unwrap();
    let _global_header =
        goblin_core_v1::input_processor::GlobalHeader::try_variable_decode(&reader, &flags)
            .unwrap();

    let market_header = goblin_core_v1::market::MarketHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(market_header.outer_bitmap_count, 1);

    // Skip market locator
    let _market_index = u8::raw_fixed_decode(&reader);

    let outer_header =
        goblin_core_v1::market::OuterBitmapHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(outer_header.outer_bitmap_index.inner, 0);
    assert_eq!(outer_header.inner_bitmap_count, 1);

    let inner_header =
        goblin_core_v1::market::InnerBitmapHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(inner_header.outer_pos.inner, 0);
    assert_eq!(inner_header.update_count, 5);

    // 1. Open Base 10
    let m1 = goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(m1.inner_pos.inner, 1);
    assert_eq!(
        m1.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Vacant
    );
    assert_eq!(m1.inner_enum_raw, false); // Base
    assert_eq!(m1.base_lots.inner, 10);

    // 2. Open Quote 20
    let m2 = goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(m2.inner_pos.inner, 2);
    assert_eq!(
        m2.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Vacant
    );
    assert_eq!(m2.inner_enum_raw, true); // Quote
    assert_eq!(m2.base_lots.inner, 20);

    // 3. Increase 30
    let m3 = goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(m3.inner_pos.inner, 3);
    assert_eq!(
        m3.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Occupied
    );
    assert_eq!(m3.inner_enum_raw, false); // Increase
    assert_eq!(m3.base_lots.inner, 30);

    // 4. Decrease 15
    let m4 = goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(m4.inner_pos.inner, 4);
    assert_eq!(
        m4.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Occupied
    );
    assert_eq!(m4.inner_enum_raw, true); // Decrease
    assert_eq!(m4.base_lots.inner, 15);

    // 5. Close 40 (encodes as Occupied Decrease)
    let m5 = goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).unwrap();
    assert_eq!(m5.inner_pos.inner, 5);
    assert_eq!(
        m5.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Occupied
    );
    assert_eq!(m5.inner_enum_raw, true); // Decrease
    assert_eq!(m5.base_lots.inner, 40);
}

#[test]
fn test_errors_and_limits() {
    let mut builder = GoblinCalldataBuilder::new();

    // 1. Custom token limit (max 7)
    for i in 0..7 {
        assert!(builder.add_custom_token([i as u8; 20]).is_ok());
    }
    assert_eq!(
        builder.add_custom_token([8; 20]),
        Err(GoblinSdkError::CustomTokenLimitExceeded(8))
    );

    // 2. Invalid hex address
    assert!(builder.set_recipient_hex("0x123").is_err());
    assert!(builder.set_recipient_hex("invalid-hex-string").is_err());

    // 3. Outer bitmap limit (max 3)
    let mut market = MarketCallBuilder::new();
    market
        .hardcoded(MarketSpecIndex::HardcodedEthHardcodedERC20, 0)
        .unwrap();
    market.make_open(
        position_from_parts(OuterBitmapIndex::new(0), OuterPos::new(0), InnerPos::new(0)),
        LegEnum::Base,
        BaseLots::new(1),
    );
    market.make_open(
        position_from_parts(OuterBitmapIndex::new(1), OuterPos::new(0), InnerPos::new(0)),
        LegEnum::Base,
        BaseLots::new(1),
    );
    market.make_open(
        position_from_parts(OuterBitmapIndex::new(2), OuterPos::new(0), InnerPos::new(0)),
        LegEnum::Base,
        BaseLots::new(1),
    );
    market.make_open(
        position_from_parts(OuterBitmapIndex::new(3), OuterPos::new(0), InnerPos::new(0)),
        LegEnum::Base,
        BaseLots::new(1),
    ); // 4th outer bitmap
    let m_call = market.build().unwrap();

    let mut b2 = GoblinCalldataBuilder::new();
    b2.add_market(m_call);
    assert_eq!(b2.build(), Err(GoblinSdkError::OuterBitmapLimitExceeded(4)));

    // 4. Invalid lot size (not dividing 1_000_000)
    let mut dynamic_builder = MarketCallBuilder::new();
    assert!(dynamic_builder
        .dynamic(
            TokenKind::ETH,
            0,
            TokenKind::HardcodedERC20,
            0,
            BaseLotsPerBaseUnit::new(777), // does not divide 1_000_000
            QuoteLotsPerQuoteUnit::new(1000),
            QuoteLotsPerBaseUnitPerTick::new(1),
        )
        .is_err());

    // 5. Zero take lots
    let mut m_zero_take = MarketCallBuilder::new();
    m_zero_take
        .hardcoded(MarketSpecIndex::HardcodedEthHardcodedERC20, 0)
        .unwrap();
    m_zero_take.take_base(BaseLots::new(0), None, None);
    let mut b3 = GoblinCalldataBuilder::new();
    b3.add_market(m_zero_take.build().unwrap());
    assert_eq!(b3.build(), Err(GoblinSdkError::ZeroTakeLots));
}
