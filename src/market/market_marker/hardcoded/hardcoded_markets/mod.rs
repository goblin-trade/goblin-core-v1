pub mod hardcoded_markets;
pub mod illegal;

pub use hardcoded_markets::*;

#[cfg(feature = "localnet")]
pub mod localnet;
#[cfg(feature = "mainnet")]
pub mod mainnet;
#[cfg(feature = "testnet")]
pub mod testnet;
