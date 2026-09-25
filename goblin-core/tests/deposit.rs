use goblin_core::{
    axis::token::{
        CustomERC20, TokenDataTriple, token_list::CustomERC20List, token_marker::TokenData,
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    goblin_error::GoblinError,
    input_processor::{GlobalArgs, Header, HeaderFlags, HeaderRefs, MarketCounts},
    quantities::UnsidedAtoms,
};
use goblin_hostio::hostio_unsafe::{set_msg_reentrant, set_test_args};

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

    let mut buffer = [0u8; 512];
    let writer = &mut Writer::new(buffer.as_mut());

    global_args.to_writer(writer, ()).unwrap();

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    // A reentrant call is rejected before any state is touched.
    set_msg_reentrant(true);
    assert!(matches!(entrypoint(), Err(GoblinError::Reentrant)));
    set_msg_reentrant(false);

    // The full decode -> process -> flush path succeeds.
    assert!(matches!(entrypoint(), Ok(())));
}
