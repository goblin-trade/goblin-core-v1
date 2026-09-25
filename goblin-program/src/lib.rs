//! Goblin Stylus program — the wasm shell only.
//!
//! This crate compiles to the wasm smart contract (`cdylib`). It carries the
//! ABI glue and the wasm-only runtime bits (panic handler, `mark_used`), and
//! delegates the actual call handling to [`goblin_core::entrypoint`].

#![cfg_attr(all(not(test), target_arch = "wasm32"), no_std)]
#![cfg_attr(all(not(test), target_arch = "wasm32"), no_main)]

mod stylus;
pub mod user_entrypoint;
