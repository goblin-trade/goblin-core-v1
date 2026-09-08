mod illegal;

#[cfg(feature = "localnet")]
mod localnet;
#[cfg(feature = "mainnet")]
mod mainnet;
#[cfg(feature = "testnet")]
mod testnet;

use crate::{axis_helpers::TokenPair, market::MarketReadables};

/// Trait to store hardcoded market lists for each B, Q combination.
/// Used with MarketLocator trait
pub trait HardcodedMarketList: 'static + TokenPair {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<Self>];
}
