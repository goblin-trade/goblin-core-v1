use goblin_core::{
    axis::{
        leg::{Base, Pair},
        market::Dynamic,
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
    market::{CommonMarket, MarketHeader},
    quantities::{
        BaseLotsPerBaseUnit, IntoAbs, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit,
        UnsidedAtoms, UnsidedLots,
    },
    settlement::LocalDeposits,
    state::{Preimage, StorePreimage},
    types::{SameTriple, StoreReader},
};
use goblin_hostio::hostio_unsafe::set_test_args;

mod test_utils;
use test_utils::{
    CUSTOM_TOKEN, CUSTOM_TOKEN_DECIMALS, MSG_SENDER, mock_decimals, mock_transfer_from, set_sender,
};

#[test]
fn test_deposit_custom_erc20() {
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
    // No hardcoded markets are processed.
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

    // The dynamic market locator is read straight out of calldata, so the test
    // builds the same `CommonMarket` the decoder expects and deposits on the
    // base (custom ERC20) leg.
    let common_market = CommonMarket::<Pair<CustomERC20, ETH>>::new(
        Pair::new(CustomERC20Index::from(0), ETHStub),
        Pair::new(
            BaseLotsPerBaseUnit::new(100),
            QuoteLotsPerQuoteUnit::new(100),
        ),
        QuoteLotsPerBaseUnitPerTick::new(1),
    );

    let deposit_lots_delta = UnsidedLots::new(10);
    let local_deposits = LocalDeposits::<Pair<CustomERC20, ETH>>::new(deposit_lots_delta, ETHStub);

    // The locator is the `CommonMarket` itself. The slot key is never sent on
    // the wire; the contract derives it from the token addresses when it
    // resolves the market.
    let market_header = MarketHeader::<(Dynamic, Pair<CustomERC20, ETH>)> {
        decode_deposit_amounts: true,
        execute_takes: Pair::new(false, false),
        outer_bitmap_count: 0,
        local_deposits,
        locator: common_market,
    };

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    // 1. Set calldata
    global_args.to_writer(writer, ()).unwrap();
    market_header.to_writer(writer, ()).unwrap();

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    // 2. Set msg_sender
    set_sender();

    // 3. The custom token's decimals are read via hostio, and the deposit is
    //    pulled with `transferFrom`. Both mocks are keyed by (call type, token
    //    address, calldata), so they only apply to this token.
    mock_decimals(custom_token_data.address, CUSTOM_TOKEN_DECIMALS);
    mock_transfer_from(custom_token_data.address);

    // The full decode -> process -> flush path succeeds.
    assert!(matches!(entrypoint(), Ok(())));

    let hash = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: custom_token_data.address,
    }
    .hash();

    let store = hash.load();

    let atoms_per_lot_pair = market_header.locator.atoms_per_lot_pair();
    let atoms_per_lot = Base::get(&atoms_per_lot_pair);

    let deposit_lots = deposit_lots_delta.abs();
    let deposit_atoms = deposit_lots * atoms_per_lot;

    assert_eq!(store.atoms_free, deposit_atoms);
    assert_eq!(store.atoms_locked, UnsidedAtoms::default());
    assert_eq!(store.decimals, CUSTOM_TOKEN_DECIMALS);
}
