use goblin_core::{
    axis::token::{
        CustomERC20, ETH, ETHStub, TokenDataTriple, token_list::CustomERC20List,
        token_marker::TokenData,
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::{GlobalArgs, Header, HeaderFlags, HeaderRefs, INPUT_SIZE, MarketCounts},
    quantities::{ETHAtoms, UnsidedAtoms},
    state::{Preimage, StorePreimage},
};
use goblin_hostio::hostio_unsafe::{set_msg_value, set_test_args};

mod test_utils;
use test_utils::{MSG_SENDER, set_sender};

/// Deposit-call calldata: turn on `read_msg_value` and nothing else, so the
/// global header is just two zero bytes of market counts.
///
/// NOTE: this test drives the global `VMContext` and `StaticDelta` singletons, so
/// it must not be split across tests that run concurrently.
#[test]
fn test_deposit_eth() {
    let custom_erc20_list_inner: [TokenData<CustomERC20>; 0] = [];
    let custom_erc20_count = custom_erc20_list_inner.len();
    let custom_erc20_list = CustomERC20List {
        inner: custom_erc20_list_inner.as_ref(),
    };
    let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

    let global_args = GlobalArgs {
        flags: HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: true,
            process_dynamic_markets: false,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count,
        },
        header: Header {
            eth_out_due_u32: UnsidedAtoms::default(),
            market_counts: MarketCounts::default(),
        },
        refs: HeaderRefs {
            custom_recipient: None,
            token_data_triple,
        },
    };

    let mut buffer = [0u8; INPUT_SIZE];
    let writer = &mut Writer::new(buffer.as_mut());

    // 1. Set calldata
    global_args.to_writer(writer, ()).unwrap();
    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    // 2. Set msg_sender
    set_sender();

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
