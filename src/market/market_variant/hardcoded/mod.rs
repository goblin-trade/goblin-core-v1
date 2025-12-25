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

pub mod dangerous_market_index;
pub mod hardcoded;
pub mod hardcoded_market_list;

pub use dangerous_market_index::*;
pub use hardcoded::*;
pub use hardcoded_market_list::*;
