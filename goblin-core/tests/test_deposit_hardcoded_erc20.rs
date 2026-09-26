use goblin_core::{
    axis::token::{
        CustomERC20, ETH, ETHStub, TokenDataTriple, token_list::CustomERC20List,
        token_marker::TokenData,
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, MarketCounts, MarketCountsInner,
    },
    quantities::{ETHAtoms, UnsidedAtoms},
    state::{Preimage, StorePreimage},
    types::SameTriple,
};
use goblin_hostio::hostio_unsafe::{set_msg_sender, set_msg_value, set_test_args};
use hex_literal::hex;

const MSG_SENDER: [u8; 20] = hex!("11D05b50ac23f0F24F536315174f35E96d2D5354");

/// Deposit-call calldata: turn on `read_msg_value` and nothing else, so the
/// global header is just two zero bytes of market counts.
///
/// NOTE: this test drives the global `VMContext` and `StaticDelta` singletons, so
/// it must not be split across tests that run concurrently.
#[test]
fn test_deposit_hardcoded_erc20() {
    let custom_erc20_list_inner: [TokenData<CustomERC20>; 0] = [];
    let custom_erc20_count = custom_erc20_list_inner.len();
    let custom_erc20_list = CustomERC20List {
        inner: custom_erc20_list_inner.as_ref(),
    };
    let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

    // One hardcoded market for `Pair<HardcodedERC20, HardcodedERC20>`.
    // Legs are indexed `[market][base][quote]`, each ordered by variant:
    // markets `[Hardcoded, Dynamic]`, tokens `[ETH, HardcodedERC20, CustomERC20]`.
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

    let mut buffer = [0u8; 512];
    let writer = &mut Writer::new(buffer.as_mut());

    // 1. Set calldata
    global_args.to_writer(writer, ()).unwrap();
    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    // 2. Set msg_sender
    set_msg_sender(MSG_SENDER);

    // 3. Set msg_value
    let eth_value = UnsidedAtoms::new(10);
    let eth_raw_atoms = ETHAtoms::try_from(eth_value).unwrap();
    set_msg_value(eth_raw_atoms.0);

    // The full decode -> process -> flush path succeeds.
    assert!(matches!(entrypoint(), Ok(())));

    let hash = StorePreimage::<ETH> {
        trader: MSG_SENDER,
        token_address: ETHStub,
    }
    .hash();

    let store = hash.load();

    assert_eq!(store.atoms_free, eth_value);
    assert_eq!(store.atoms_locked, UnsidedAtoms::default());
}
