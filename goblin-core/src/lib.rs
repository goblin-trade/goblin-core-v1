#![allow(static_mut_refs)]
#![no_std]
#![feature(const_trait_impl)]
#![feature(const_index)]
#![feature(const_try)]

#[cfg(test)]
extern crate std;

pub mod axis;
pub mod axis_helpers;
pub mod ctx;
pub mod goblin_error;
pub mod hostio;
pub mod input_processor;
pub mod instructions;
pub mod market;
pub mod matching;
pub mod quantities;
pub mod settlement;
pub mod state;
pub mod types;
pub use ctx::*;

use hex_literal::hex;

/// Address of the deployed Goblin contract (deterministic CREATE3 deployment).
///
/// The contract logic reads this at compile time, e.g. to approve
/// `transferFrom` pulls from the maker.
pub const CONTRACT_ADDRESS: [u8; 20] = hex!("8888ef09a63b6328468fce63a09fc185de807722");
