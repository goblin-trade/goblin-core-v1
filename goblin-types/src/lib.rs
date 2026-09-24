#![no_std]

//! Foundational, dependency-light types shared across the Goblin crates.
//!
//! This crate is the bottom of the dependency graph: it must not depend on
//! `goblin-core`. It currently hosts [`goblin_error`] and the wire [`codec`]
//! traits. The quantity and `types` modules are intended to move here too; see
//! the module docs on why they have not landed yet.

pub mod codec;
pub mod goblin_error;

pub use codec::*;
pub use goblin_error::*;
