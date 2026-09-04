//! # goblin-sdk-rs
//!
//! SDK for serializing calldata to call the `goblin-core-v1` Stylus smart contract.
//!
//! This crate supports both native Rust servers/backends and JavaScript/TypeScript frontends (via WebAssembly).
//! It uses strongly-typed domain primitives (`Address`, `Position`, `BaseLots`, `QuoteLots`, etc.) imported
//! directly from `goblin-core-v1` to ensure unit and dimensional correctness.
//!
//! ## Key Features
//! - Global arguments encoding: flags, custom recipient, msg.value, internal/external ETH withdrawals, custom ERC20 token registration.
//! - Multi-market batching across all 11 legal market specifications (Hardcoded and Dynamic).
//! - Strongly typed market operations: local deposits, limit/market take orders, and make orders (open, increase, decrease, close).
//! - Automatic 2-level bitmap hierarchy grouping for make orders.
//! - WASM bindings (`wasm-bindgen`) for browser and Node.js environments.

pub mod builder;
pub mod encoder;
pub mod error;
pub mod position;
pub mod types;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use builder::{GoblinCalldataBuilder, MarketCallBuilder};
pub use encoder::{GlobalPayloadConfig, GoblinEncoder};
pub use error::GoblinSdkError;
pub use position::{
    column_from_position, group_makes_by_bitmaps, parts_from_position, position_from_parts,
    position_from_ticks, ticks_from_position, InnerBitmapGroup, OuterBitmapGroup,
};
pub use types::{
    BaseTakeOrder, MakeAction, MarketCall, MarketDeposits, MarketKind, MarketLocator,
    MarketSpecIndex, PositionedMakeOrder, QuoteTakeOrder, TakeOrder, TokenKind,
};

// Re-export strongly typed domain primitives from goblin-core-v1
pub use goblin_core_v1::{
    axis::leg::LegEnum,
    quantities::{
        BaseLots, BaseLotsPerBaseUnit, Column, InnerPos, OuterBitmapIndex, OuterPos, Position,
        QuoteLots, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, Ticks, UnsidedAtoms,
        UnsidedDeltaLots,
    },
    types::Address,
};
