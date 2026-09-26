//! A custom-ERC20 deposit into a fresh dynamic market.
//!
//! A custom ERC20 can never live in a hardcoded market, so it is listed in
//! calldata and traded through a dynamic `Pair<CustomERC20, ETH>` market whose
//! locator is read straight out of calldata. The market fixture is shared with
//! the make tests via `tests/test_utils`.

use goblin_core::{
    axis::{leg::Base, token::CustomERC20},
    codec::{GoblinWrite, Writer},
    entrypoint,
    input_processor::INPUT_SIZE,
    quantities::{IntoAbs, UnsidedAtoms, UnsidedLots},
    state::{Preimage, StorePreimage},
    types::StoreReader,
};
use goblin_hostio::hostio_unsafe::set_test_args;

use crate::test_utils::{
    CUSTOM_TOKEN, CUSTOM_TOKEN_DECIMALS, MSG_SENDER, custom_erc20_eth_global_args,
    custom_erc20_eth_market_header, custom_erc20_list_inner, custom_market_token_data_triple,
    isolated, mock_custom_token, set_sender,
};

/// Base lots deposited into the dynamic market.
const DEPOSIT_LOTS: i64 = 10;

#[test]
fn test_deposit_custom_erc20() {
    let _guard = isolated();

    let custom_tokens = custom_erc20_list_inner();
    let token_data_triple = custom_market_token_data_triple(&custom_tokens);

    let global_args = custom_erc20_eth_global_args(token_data_triple);
    // A pure deposit is not followed by any make or take headers.
    let market_header = custom_erc20_eth_market_header(DEPOSIT_LOTS, 0);

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
    //    pulled with `transferFrom`.
    mock_custom_token();

    // The full decode -> process -> flush path succeeds.
    assert!(matches!(entrypoint(), Ok(())));

    let store = StorePreimage::<CustomERC20> {
        trader: MSG_SENDER,
        token_address: CUSTOM_TOKEN,
    }
    .hash()
    .load();

    let atoms_per_lot = Base::get(&market_header.locator.atoms_per_lot_pair());

    let deposit_lots = UnsidedLots::new(DEPOSIT_LOTS).abs();
    let deposit_atoms = deposit_lots * atoms_per_lot;

    assert_eq!(store.atoms_free, deposit_atoms);
    assert_eq!(store.atoms_locked, UnsidedAtoms::default());
    assert_eq!(store.decimals, CUSTOM_TOKEN_DECIMALS);
}
