use goblin_core_v1::input_processor::{
    ArgsReader, CompoundDecode, FixedDecode, HeaderFlags, VariableDecode,
};
use goblin_sdk_rs::{
    column_from_position, parts_from_position, position_from_parts, position_from_ticks,
    ticks_from_position, GoblinCalldataBuilder, LegSide, MarketCallBuilder, MarketSpecIndex,
    TokenKind,
};

#[test]
fn test_position_math_roundtrip() {
    let position: u64 = 0x1234_5678_9abc_def0;
    let (outer_bitmap_idx, outer_pos, inner_pos) = parts_from_position(position);
    let reconstructed = position_from_parts(outer_bitmap_idx, outer_pos, inner_pos);
    assert_eq!(position, reconstructed);

    let ticks = ticks_from_position(position);
    let column = column_from_position(position);
    let reconstructed_ticks = position_from_ticks(ticks, column);
    assert_eq!(position, reconstructed_ticks);
}

#[test]
fn test_global_header_decoding() {
    let recipient = [0x11u8; 20];
    let custom_token_1 = [0x22u8; 20];
    let custom_token_2 = [0x33u8; 20];

    let mut builder = GoblinCalldataBuilder::new();
    builder
        .set_recipient(recipient)
        .set_msg_value(true)
        .set_eth_withdrawal(500_000, true);

    let token_idx_1 = builder.add_custom_token(custom_token_1).unwrap();
    let token_idx_2 = builder.add_custom_token(custom_token_2).unwrap();
    assert_eq!(token_idx_1, 0);
    assert_eq!(token_idx_2, 1);

    let calldata = builder.build().expect("build failed");

    // Decode with goblin-core-v1
    let reader = ArgsReader::from_slice(&calldata);
    let flags = HeaderFlags::try_fixed_decode(&reader).expect("failed to decode flags");

    assert!(flags.read_custom_recipient);
    assert!(flags.read_msg_value);
    assert!(!flags.process_dynamic_markets);
    assert!(flags.withdraw_eth);
    assert!(flags.withdraw_internally);
    assert_eq!(flags.custom_erc20_count, 2);

    let global_header =
        goblin_core_v1::input_processor::GlobalHeader::try_variable_decode(&reader, &flags)
            .expect("failed to decode global header");

    assert_eq!(global_header.eth_out_due.inner, 500_000);
    assert_eq!(global_header.custom_recipient, Some(&recipient));
}

#[test]
fn test_hardcoded_market_with_takes_and_makes() {
    let mut builder = GoblinCalldataBuilder::new();

    // Spec #0: Hardcoded (ETH, HardcodedERC20)
    let mut market_builder = MarketCallBuilder::new();
    market_builder
        .hardcoded(MarketSpecIndex::HardcodedEthHardcodedERC20, 0)
        .unwrap()
        .deposit(0, 1000) // quote deposit (HardcodedERC20 is quote)
        .take_base(50, Some(40), Some(1_000_000))
        .take_quote(100, None, None)
        .make_open(position_from_parts(1, 2, 3), LegSide::Base, 20)
        .make_open(position_from_parts(1, 2, 4), LegSide::Quote, 30)
        .make_increase(position_from_parts(1, 5, 6), 15);

    let market_call = market_builder.build().unwrap();
    builder.add_market(market_call);

    let calldata = builder.build().expect("build failed");

    // Verify calldata can be decoded
    let reader = ArgsReader::from_slice(&calldata);
    let flags = HeaderFlags::try_fixed_decode(&reader).expect("failed to decode flags");
    assert!(!flags.process_dynamic_markets);
    assert_eq!(flags.custom_erc20_count, 0);

    let global_header =
        goblin_core_v1::input_processor::GlobalHeader::try_variable_decode(&reader, &flags)
            .expect("failed to decode global header");

    // First hardcoded market count should be 1
    assert_eq!(global_header.market_counts.get_count_and_advance(), 1);

    // Decode MarketHeader
    let market_header = goblin_core_v1::market::MarketHeader::try_fixed_decode(&reader)
        .expect("decode market header");
    assert!(market_header.decode_deposit_amounts);
    assert!(market_header.execute_takes.0); // base take
    assert!(market_header.execute_takes.1); // quote take
    assert_eq!(market_header.outer_bitmap_count, 1);

    // Decode deposits: ETH base has 0 bytes, HardcodedERC20 quote has 8 bytes
    let quote_deposit = i64::raw_fixed_decode(&reader);
    assert_eq!(quote_deposit, 1000);

    // Decode market locator: market index 0 (1 byte)
    let market_index = u8::raw_fixed_decode(&reader);
    assert_eq!(market_index, 0);

    // Decode base take order
    let base_take = goblin_core_v1::instructions::take::take_header::TakeHeader::<
        goblin_core_v1::axis::leg::Base,
    >::try_compound_decode(&reader)
    .expect("decode base take");
    assert_eq!(base_take.num_lots.inner, 50);
    assert_eq!(base_take.min_lots_to_fill.inner, 40);
    assert_eq!(base_take.limit.inner, 1_000_000);

    // Decode quote take order
    let quote_take = goblin_core_v1::instructions::take::take_header::TakeHeader::<
        goblin_core_v1::axis::leg::Quote,
    >::try_compound_decode(&reader)
    .expect("decode quote take");
    assert_eq!(quote_take.num_lots.inner, 100);
    assert_eq!(quote_take.min_lots_to_fill.inner, 0);

    // Decode OuterBitmapHeader
    let outer_header = goblin_core_v1::market::OuterBitmapHeader::try_fixed_decode(&reader)
        .expect("decode outer bitmap header");
    assert_eq!(outer_header.outer_bitmap_index.inner, 1);
    assert_eq!(outer_header.inner_bitmap_count, 2); // outer_pos 2 and 5

    // Inner bitmap 1 (outer_pos 2)
    let inner_header_1 = goblin_core_v1::market::InnerBitmapHeader::try_fixed_decode(&reader)
        .expect("decode inner bitmap header 1");
    assert_eq!(inner_header_1.outer_pos.inner, 2);
    assert_eq!(inner_header_1.update_count, 2);

    // Make 1 (inner_pos 3, open base 20)
    let make_1 =
        goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).expect("decode make 1");
    assert_eq!(make_1.inner_pos.inner, 3);
    assert_eq!(
        make_1.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Vacant
    );
    assert_eq!(make_1.inner_enum_raw, false); // Base
    assert_eq!(make_1.base_lots.inner, 20);

    // Make 2 (inner_pos 4, open quote 30)
    let make_2 =
        goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).expect("decode make 2");
    assert_eq!(make_2.inner_pos.inner, 4);
    assert_eq!(
        make_2.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Vacant
    );
    assert_eq!(make_2.inner_enum_raw, true); // Quote
    assert_eq!(make_2.base_lots.inner, 30);

    // Inner bitmap 2 (outer_pos 5)
    let inner_header_2 = goblin_core_v1::market::InnerBitmapHeader::try_fixed_decode(&reader)
        .expect("decode inner bitmap header 2");
    assert_eq!(inner_header_2.outer_pos.inner, 5);
    assert_eq!(inner_header_2.update_count, 1);

    // Make 3 (inner_pos 6, increase 15)
    let make_3 =
        goblin_core_v1::market::MakeHeader::try_fixed_decode(&reader).expect("decode make 3");
    assert_eq!(make_3.inner_pos.inner, 6);
    assert_eq!(
        make_3.occupancy_enum,
        goblin_core_v1::axis::occupancy::OccupancyEnum::Occupied
    );
    assert_eq!(make_3.inner_enum_raw, false); // Increase
    assert_eq!(make_3.base_lots.inner, 15);
}

#[test]
fn test_dynamic_market_batching() {
    let mut builder = GoblinCalldataBuilder::new();
    let token_a = [0xaa; 20];
    let token_b = [0xbb; 20];

    let idx_a = builder.add_custom_token(token_a).unwrap();
    let idx_b = builder.add_custom_token(token_b).unwrap();

    // Spec #10: Dynamic (CustomERC20, CustomERC20)
    let mut market = MarketCallBuilder::new();
    market
        .dynamic(
            TokenKind::CustomERC20,
            idx_a,
            TokenKind::CustomERC20,
            idx_b,
            100,  // base lot size
            1000, // quote lot size
            1,    // tick size
        )
        .unwrap()
        .deposit(500, 600)
        .take_base(25, None, None);

    builder.add_market(market.build().unwrap());

    let calldata = builder.build().expect("build dynamic calldata");
    assert!(!calldata.is_empty());

    let reader = ArgsReader::from_slice(&calldata);
    let flags = HeaderFlags::try_fixed_decode(&reader).expect("decode flags");
    assert!(flags.process_dynamic_markets);
    assert_eq!(flags.custom_erc20_count, 2);

    let global_header =
        goblin_core_v1::input_processor::GlobalHeader::try_variable_decode(&reader, &flags)
            .expect("decode global header");

    // All hardcoded counts should be 0
    assert_eq!(global_header.market_counts.get_count_and_advance(), 0);
    assert_eq!(global_header.market_counts.get_count_and_advance(), 0);
    assert_eq!(global_header.market_counts.get_count_and_advance(), 0);

    // Dynamic specs 3..9 should be 0
    for _ in 3..10 {
        assert_eq!(global_header.market_counts.get_count_and_advance(), 0);
    }
    // Spec 10 count should be 1
    assert_eq!(global_header.market_counts.get_count_and_advance(), 1);
}
