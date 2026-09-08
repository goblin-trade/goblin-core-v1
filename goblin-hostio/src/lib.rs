//! Low-level Stylus hostio bindings and the native "test hostio" emulation.
//!
//! This crate is intentionally independent of `goblin-core` so that it can be
//! placed underneath it in the dependency graph.

#![cfg_attr(target_arch = "wasm32", no_std)]

pub mod hostio_helpers;
pub mod hostio_unsafe;
