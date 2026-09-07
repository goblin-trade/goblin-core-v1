//! Goblin Stylus program. This crate compiles to the wasm smart contract
//! (`cdylib`), wiring the low-level hostio bindings (`goblin-hostio`) to the
//! pure trading logic (`goblin-core`).

#![cfg_attr(all(not(test), target_arch = "wasm32"), no_std)]
#![cfg_attr(all(not(test), target_arch = "wasm32"), no_main)]

mod stylus;
pub mod user_entrypoint;
