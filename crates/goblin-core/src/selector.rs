//! Goblin uses a custom selector, which is a one byte number unlike the 4
//! byte selector used in Ethereum.

// A market is defined by the following fields:
// - base token
// - quote token
// - fee
// - base decimals to ignore
// - quote decimals to ignore

// depositFunds()
pub const DEPOSIT_FUNDS_SELECTOR: u8 = 0;

pub const SET_COUNT_SELECTOR: u8 = 1;

pub const GET_COUNT_SELECTOR: u8 = 2;
