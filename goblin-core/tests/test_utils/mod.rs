//! Shared fixtures for the `goblin-core` integration tests.
//!
//! Test-specific hardcoded addresses, token ABIs and hostio mocks live here so
//! the integration tests do not each re-declare (and slowly drift from) the same
//! values.
//!
//! This module also owns the dynamic `Pair<CustomERC20, ETH>` market fixture
//! that the deposit and make tests all build.
//!
//! This is kept in `tests/test_utils/mod.rs` rather than `tests/test_utils.rs`
//! so Cargo does not compile it into its own empty test binary. Pull it into a
//! test binary with `mod test_utils;` (the test binaries that live in a
//! sub-directory have to include it by path, e.g.
//! `#[path = "../test_utils/mod.rs"] mod test_utils;`).

#![allow(dead_code)]

use std::sync::{Mutex, MutexGuard};

use alloy_sol_types::{SolCall, sol};
use goblin_core::{
    axis::{
        leg::Pair,
        market::Dynamic,
        token::{
            CustomERC20, CustomERC20Stub, ETH, ETHStub, TokenDataTriple,
            token_list::CustomERC20List,
            token_marker::{CustomERC20Index, TokenData},
        },
    },
    input_processor::{
        GlobalArgs, Header, HeaderFlags, HeaderRefs, MarketCounts, MarketCountsInner,
    },
    market::{CommonMarket, MarketHeader},
    quantities::{
        BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, UnsidedAtoms,
        UnsidedLots,
    },
    settlement::{ConstDefault, LocalDeposits, StaticDelta},
    types::SameTriple,
};
use goblin_hostio::hostio_unsafe::{
    VMContext, set_mock_call, set_mock_static_call, set_msg_sender, vm_ctx,
};
use hex_literal::hex;

/// The trader used as `msg_sender` by the integration tests.
pub const MSG_SENDER: [u8; 20] = hex!("11D05b50ac23f0F24F536315174f35E96d2D5354");

/// A custom ERC20 token that is not part of the hardcoded token list, so it can
/// only be traded in a dynamic market. This is the mainnet USDC address, used
/// purely as an opaque address.
pub const CUSTOM_TOKEN: [u8; 20] = hex!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");

/// Decimals mocked for [`CUSTOM_TOKEN`]. `update_erc20` accepts 6, 8 or 18.
pub const CUSTOM_TOKEN_DECIMALS: u8 = 6;

// The subset of the ERC20 ABI the tests exercise. Declared once here so the
// selectors and return encoders are derived rather than hand-rolled.
sol! {
    interface IERC20 {
        function transferFrom(address from, address to, uint256 amount) returns (bool);
        function decimals() external view returns (uint8);
    }
}

// ---------------------------------------------------------------------------
// Test isolation
// ---------------------------------------------------------------------------

/// Serialises tests that drive the process-global hostio and settlement state.
///
/// Every test in one integration-test binary shares a single process, so tests
/// that call `entrypoint()` would otherwise race on that shared state.
pub static SERIAL: Mutex<()> = Mutex::new(());

/// Clear the process-global hostio emulation and settlement scratch space.
///
/// [`VMContext`] and [`StaticDelta`] are process singletons: without this a test
/// would observe the storage, mocks, message fields and deltas left behind by a
/// sibling test.
pub fn reset_state() {
    *vm_ctx() = VMContext::new();
    *StaticDelta::get() = StaticDelta::DEFAULT;
}

/// Acquire [`SERIAL`] and clear the shared globals.
///
/// Hold the returned guard for the whole test so that sibling tests in the same
/// binary cannot interleave with it. Poisoning is ignored: a panicking test
/// should not wedge the rest of the suite.
pub fn isolated() -> MutexGuard<'static, ()> {
    let guard = SERIAL
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    reset_state();
    guard
}

// ---------------------------------------------------------------------------
// Hostio mocks
// ---------------------------------------------------------------------------

/// Select [`MSG_SENDER`] as the caller for the next `entrypoint()` run.
pub fn set_sender() {
    set_msg_sender(MSG_SENDER);
}

/// Mock `token.transferFrom(..)` for `token` to succeed.
pub fn mock_transfer_from(token: [u8; 20]) {
    set_mock_call(
        token,
        IERC20::transferFromCall::SELECTOR.to_vec(),
        IERC20::transferFromCall::abi_encode_returns(&true),
    );
}

/// Mock `token.decimals()` for `token` to return `decimals`.
pub fn mock_decimals(token: [u8; 20], decimals: u8) {
    set_mock_static_call(
        token,
        IERC20::decimalsCall::SELECTOR.to_vec(),
        IERC20::decimalsCall::abi_encode_returns(&decimals),
    );
}

/// Mock every hostio read the custom ERC20 market performs on [`CUSTOM_TOKEN`]:
/// `decimals()` and `transferFrom(..)`.
pub fn mock_custom_token() {
    mock_decimals(CUSTOM_TOKEN, CUSTOM_TOKEN_DECIMALS);
    mock_transfer_from(CUSTOM_TOKEN);
}

// ---------------------------------------------------------------------------
// Dynamic `Pair<CustomERC20, ETH>` market
// ---------------------------------------------------------------------------
//
// A custom ERC20 can never live in a hardcoded market, so it is listed in
// calldata and traded through a dynamic market. The deposit and make tests both
// build this same market, so its construction is centralised here.

/// The calldata token-list entry for [`CUSTOM_TOKEN`].
pub fn custom_token_data() -> TokenData<CustomERC20> {
    TokenData::<CustomERC20> {
        address: CUSTOM_TOKEN,
        decimals: CustomERC20Stub,
    }
}

/// Backing store for the calldata custom-token list.
///
/// [`CustomERC20List`] borrows a slice, so the caller must keep this array alive
/// for at least as long as the [`TokenDataTriple`] built from it.
pub fn custom_erc20_list_inner() -> [TokenData<CustomERC20>; 1] {
    [custom_token_data()]
}

/// The [`TokenDataTriple`] for the dynamic custom-ERC20 market.
pub fn custom_market_token_data_triple<'a>(
    inner: &'a [TokenData<CustomERC20>],
) -> TokenDataTriple<'a> {
    TokenDataTriple::const_from(CustomERC20List { inner })
}

/// `MarketCounts` selecting a single dynamic `Pair<CustomERC20, ETH>` market.
///
/// Legs are indexed `[market][base][quote]`, each ordered by variant: markets
/// `[Hardcoded, Dynamic]`, tokens `[ETH, HardcodedERC20, CustomERC20]`.
pub fn custom_erc20_eth_market_counts() -> MarketCounts {
    MarketCounts::new(
        MarketCountsInner::default(), // hardcoded markets
        MarketCountsInner::new(
            SameTriple::new(0, 0, 0), // base ETH
            SameTriple::new(0, 0, 0), // base HardcodedERC20
            SameTriple::new(1, 0, 0), // base CustomERC20, quote ETH
        ),
    )
}

/// Global args for a call that processes only the dynamic custom-ERC20 market.
pub fn custom_erc20_eth_global_args<'a>(token_data_triple: TokenDataTriple<'a>) -> GlobalArgs<'a> {
    GlobalArgs {
        flags: HeaderFlags {
            read_custom_recipient: false,
            read_msg_value: false,
            process_dynamic_markets: true,
            withdraw_eth: false,
            withdraw_internally: false,
            custom_erc20_count: 1,
        },
        header: Header {
            eth_out_due_u32: UnsidedAtoms::default(),
            market_counts: custom_erc20_eth_market_counts(),
        },
        refs: HeaderRefs {
            custom_recipient: None,
            token_data_triple,
        },
    }
}

/// The dynamic `Pair<CustomERC20, ETH>` market locator read from calldata.
///
/// The locator is not sent as a slot key; the contract derives the market key
/// from the token addresses, and tests do the same via
/// [`CommonMarket::get_preimage`].
pub fn custom_erc20_eth_market() -> CommonMarket<Pair<CustomERC20, ETH>> {
    CommonMarket::<Pair<CustomERC20, ETH>>::new(
        Pair::new(CustomERC20Index::from(0), ETHStub),
        Pair::new(
            BaseLotsPerBaseUnit::new(100),
            QuoteLotsPerQuoteUnit::new(100),
        ),
        QuoteLotsPerBaseUnitPerTick::new(1),
    )
}

/// A [`MarketHeader`] for the dynamic custom-ERC20 market.
///
/// `deposit_lots_delta` is the local base (custom ERC20) deposit; `outer_bitmap_count`
/// is the number of make/outer-bitmap headers that follow.
pub fn custom_erc20_eth_market_header(
    deposit_lots_delta: i64,
    outer_bitmap_count: u8,
) -> MarketHeader<(Dynamic, Pair<CustomERC20, ETH>)> {
    MarketHeader::<(Dynamic, Pair<CustomERC20, ETH>)> {
        decode_deposit_amounts: true,
        execute_takes: Pair::new(false, false),
        outer_bitmap_count,
        local_deposits: LocalDeposits::<Pair<CustomERC20, ETH>>::new(
            UnsidedLots::new(deposit_lots_delta),
            ETHStub,
        ),
        locator: custom_erc20_eth_market(),
    }
}
