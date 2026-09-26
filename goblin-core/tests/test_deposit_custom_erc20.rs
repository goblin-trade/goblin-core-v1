use alloy_sol_types::{SolCall, sol};
use goblin_core::{
    axis::{
        leg::{Base, Pair},
        token::{
            CustomERC20, CustomERC20Stub, ETH, ETHStub, TokenDataTriple,
            token_list::CustomERC20List,
            token_marker::{CustomERC20Index, TokenData},
        },
    },
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, MarketCounts, MarketCountsInner, write_lane,
    },
    market::CommonMarket,
    quantities::{
        BaseLotsPerBaseUnit, IntoAbs, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit,
        UnsidedAtoms, UnsidedLots,
    },
    settlement::LocalDeposits,
    state::{Preimage, StorePreimage},
    types::{SameTriple, StoreReader},
};
use goblin_hostio::hostio_unsafe::{
    set_mock_call, set_mock_static_call, set_msg_sender, set_test_args,
};
use hex_literal::hex;

const MSG_SENDER: [u8; 20] = hex!("11D05b50ac23f0F24F536315174f35E96d2D5354");

/// A custom ERC20 token that is not part of the hardcoded token list, so it can
/// only be traded in a dynamic market.
const CUSTOM_TOKEN: [u8; 20] = hex!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

/// Decimals mocked for the custom token. `update_erc20` accepts 6, 8 or 18.
const CUSTOM_TOKEN_DECIMALS: u8 = 6;

sol! {
    interface IERC20 {
        function transferFrom(address from, address to, uint256 amount) returns (bool);
        function decimals() external view returns (uint8);
    }
}

#[test]
fn test_deposit_custom_erc20() {
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

    let market_counts = MarketCounts::new(
        MarketCountsInner::default(),
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

    let deposit_lots_delta = UnsidedLots::new(10);
    let local_deposits = LocalDeposits::<Pair<CustomERC20, ETH>>::new(deposit_lots_delta, ETHStub);

    let mut buffer = [0u8; 512];
    let writer = &mut Writer::new(buffer.as_mut());

    // 1. Set calldata
    global_args.to_writer(writer, ()).unwrap();

    // Dynamic markets are not written by `MarketHeader` (their locator is a
    // `MarketReadables`, which has no writer), so the per-market header is laid
    // out by hand: a packed lane with `decode_deposit_amounts` set, the
    // `CommonMarket` locator, then the local deposits.
    write_lane::<1>(writer, 0b1).unwrap();
    common_market.to_writer(writer, ()).unwrap();
    local_deposits.to_writer(writer, ()).unwrap();

    let calldata = writer.get_calldata();
    set_test_args(calldata.to_vec());

    // 2. Set msg_sender
    set_msg_sender(MSG_SENDER);

    // 3. The custom token's decimals are read via hostio, and the deposit is
    //    pulled with `transferFrom`. Both mocks are keyed by (call type, token
    //    address, calldata), so they only apply to this token.
    set_mock_static_call(
        custom_token_data.address,
        IERC20::decimalsCall::SELECTOR.to_vec(),
        IERC20::decimalsCall::abi_encode_returns(&CUSTOM_TOKEN_DECIMALS),
    );

    set_mock_call(
        custom_token_data.address,
        IERC20::transferFromCall::SELECTOR.to_vec(),
        IERC20::transferFromCall::abi_encode_returns(&true),
    );

    // The full decode -> process -> flush path succeeds.
    assert!(matches!(entrypoint(), Ok(())));

    let hash = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: custom_token_data.address,
    }
    .hash();

    let store = hash.load();

    let atoms_per_lot_pair = common_market.atoms_per_lot_pair();
    let atoms_per_lot = Base::get(&atoms_per_lot_pair);

    let deposit_lots = deposit_lots_delta.abs();
    let deposit_atoms = deposit_lots * atoms_per_lot;

    assert_eq!(store.atoms_free, deposit_atoms);
    assert_eq!(store.atoms_locked, UnsidedAtoms::default());
    assert_eq!(store.decimals, CUSTOM_TOKEN_DECIMALS);
}
