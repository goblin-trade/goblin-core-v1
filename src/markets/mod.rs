pub mod indexed_market;
pub use indexed_market::*;
pub mod market_index;
pub use market_index::*;
pub mod market_instructions;
pub use market_instructions::*;

#[cfg(feature = "localnet")]
pub mod localnet;
#[cfg(feature = "mainnet")]
pub mod mainnet;
#[cfg(feature = "testnet")]
pub mod testnet;

#[cfg(feature = "localnet")]
pub use localnet::*;
#[cfg(feature = "mainnet")]
pub use mainnet::*;
#[cfg(feature = "testnet")]
pub use testnet::*;
