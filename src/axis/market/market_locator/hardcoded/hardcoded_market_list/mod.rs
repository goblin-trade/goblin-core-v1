pub mod hardcoded_market_list;
pub use hardcoded_market_list::*;

pub mod illegal;

#[cfg(feature = "localnet")]
pub mod localnet;
#[cfg(feature = "mainnet")]
pub mod mainnet;
#[cfg(feature = "testnet")]
pub mod testnet;
