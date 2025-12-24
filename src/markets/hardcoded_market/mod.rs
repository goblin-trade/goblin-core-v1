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

mod decoder;
pub mod hardcoded_market;
pub mod hardcoded_market_index;
pub mod hardcoded_market_list;

pub use hardcoded_market::*;
pub use hardcoded_market_index::*;
pub use hardcoded_market_list::*;
