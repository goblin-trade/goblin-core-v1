#![allow(static_mut_refs)]
#![cfg_attr(all(not(test), target_arch = "wasm32"), no_std)]
#![cfg_attr(all(not(test), target_arch = "wasm32"), no_main)]

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
mod stylus;
pub mod types;
pub mod user_entrypoint;

pub use ctx::*;
