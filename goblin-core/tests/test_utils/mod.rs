//! Shared fixtures for the `goblin-core` integration tests.
//!
//! Test-specific hardcoded addresses, token ABIs and hostio mocks live here so
//! the deposit tests do not each re-declare (and slowly drift from) the same
//! values.
//!
//! This is kept in `tests/test_utils/mod.rs` rather than `tests/test_utils.rs`
//! so Cargo does not compile it into its own empty test binary. Pull it into a
//! test with `mod test_utils;`.

#![allow(dead_code)]

use alloy_sol_types::{SolCall, sol};
use goblin_hostio::hostio_unsafe::{set_mock_call, set_mock_static_call, set_msg_sender};
use hex_literal::hex;

/// The trader used as `msg_sender` by the deposit tests.
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
