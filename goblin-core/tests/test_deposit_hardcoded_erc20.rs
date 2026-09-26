use goblin_core::hostio::abi_selector;
use goblin_core::{
    axis::{
        leg::{Base, Pair},
        market::{Hardcoded, HardcodedMarketList, MarketIndex},
        token::{
            CustomERC20, HardcodedERC20, TokenDataTriple,
            token_list::{CustomERC20List, HARDCODED_ERC20_LIST},
            token_marker::TokenData,
        },
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, MarketCounts, MarketCountsInner,
    },
    market::MarketHeader,
    quantities::{IntoAbs, UnsidedAtoms, UnsidedLots},
    settlement::LocalDeposits,
    state::{Preimage, StorePreimage},
    types::{SameTriple, StoreReader},
};
use goblin_hostio::hostio_unsafe::{set_mock_call, set_msg_sender, set_test_args};
use hex_literal::hex;

const MSG_SENDER: [u8; 20] = hex!("11D05b50ac23f0F24F536315174f35E96d2D5354");

#[test]
fn test_deposit_hardcoded_erc20() {
    let custom_erc20_list_inner: [TokenData<CustomERC20>; 0] = [];
    let custom_erc20_count = custom_erc20_list_inner.len();
    let custom_erc20_list = CustomERC20List {
        inner: custom_erc20_list_inner.as_ref(),
    };
    let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

    let market_counts = MarketCounts::new(
        MarketCountsInner::new(
            SameTriple::new(0, 0, 0), // base ETH
            SameTriple::new(0, 1, 0), // base HardcodedERC20, quote HardcodedERC20
            SameTriple::new(0, 0, 0), // base CustomERC20
        ),
        MarketCountsInner::default(), // dynamic markets
    );

    let global_args = GlobalArgs {
        flags: HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: false,
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

    let deposit_lots_delta = UnsidedLots::new(10);
    let local_deposits = LocalDeposits::<Pair<HardcodedERC20, HardcodedERC20>>::new(
        deposit_lots_delta,
        UnsidedLots::default(),
    );

    let hardcoded_market_index = MarketIndex::new(0);

    let market_header = MarketHeader::<(Hardcoded, Pair<HardcodedERC20, HardcodedERC20>)> {
        decode_deposit_amounts: true,
        execute_takes: Pair::new(false, false),
        outer_bitmap_count: 0,
        local_deposits,
        locator: hardcoded_market_index,
    };

    let mut buffer = [0u8; 512];
    let writer = &mut Writer::new(buffer.as_mut());

    // 1. Set calldata
    global_args.to_writer(writer, ()).unwrap();
    market_header.to_writer(writer, ()).unwrap();

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    // 2. Set msg_sender
    set_msg_sender(MSG_SENDER);

    // 3. ERC20 `transferFrom` succeeds for the hardcoded token being deposited.
    //    The mock is keyed by (call type, token address, calldata), so it only
    //    applies to this call and leaves any other hostio call untouched.
    let hardcoded_token_data = HARDCODED_ERC20_LIST.inner[0];

    let mut erc20_return = vec![0u8; 32];
    erc20_return[31] = 1;
    set_mock_call(
        hardcoded_token_data.address,
        abi_selector(b"transferFrom(address,address,uint256)").to_vec(),
        erc20_return,
    );

    // The full decode -> process -> flush path succeeds.
    assert!(matches!(entrypoint(), Ok(())));

    let hash = StorePreimage::<HardcodedERC20> {
        trader: MSG_SENDER,
        token_address: hardcoded_token_data.address,
    }
    .hash();

    let store = hash.load();

    let hardcoded_market =
        &<Pair<HardcodedERC20, HardcodedERC20> as HardcodedMarketList>::HARDCODED_MARKET_LIST
            [hardcoded_market_index];

    let atoms_per_lot_pair = hardcoded_market.market.atoms_per_lot_pair();
    let atoms_per_lot = Base::get(&atoms_per_lot_pair);

    let deposit_lots = deposit_lots_delta.abs();
    let deposit_atoms = deposit_lots * atoms_per_lot;

    assert_eq!(store.atoms_free, deposit_atoms);
    assert_eq!(store.atoms_locked, UnsidedAtoms::default());
    assert_eq!(store.decimals, hardcoded_token_data.decimals);
}
