pub mod hardcoded_market_list;
pub use hardcoded_market_list::*;

mod illegal;

#[cfg(feature = "localnet")]
mod localnet;
#[cfg(feature = "mainnet")]
mod mainnet;
#[cfg(feature = "testnet")]
mod testnet;
