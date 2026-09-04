//! # goblin-sdk-rs
//!
//! SDK for serializing calldata to call the `goblin-core-v1` Stylus smart contract.
//!
//! This crate supports both native Rust servers/backends and JavaScript/TypeScript frontends (via WebAssembly).
//! It provides high-level builder APIs as well as lower-level composable encoders to construct compact,
//! wire-efficient calldata payloads according to the custom encoding specifications of `goblin-core-v1`.
//!
//! ## Key Features
//! - Global arguments encoding: flags, custom recipient, msg.value, internal/external ETH withdrawals, custom ERC20 token registration.
//! - Multi-market batching across all 11 legal market specifications (Hardcoded and Dynamic).
//! - Market operations: local deposits, limit/market take orders (with optional min fill lots and price limits), and make orders (open, increase, decrease, close).
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
    LegSide, MakeAction, MarketCall, MarketDeposits, MarketKind, MarketLocator, MarketSpecIndex,
    PositionedMakeOrder, TakeOrder, TokenKind,
};
