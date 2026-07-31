use crate::axis::market::{
    market_locator::{hardcoded::HardcodedMarkets, MarketIndex, MarketLocator},
    token_pair::TokenPair,
    Hardcoded,
};

///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either hardcoded or custom
///!
///! However since we use generics, all combinations must be implented. Even hardcoded
///! markets with custom tokens.

pub trait MarketMarker: Sized + Clone + Copy {
    /// Discriminator used to hash the market key
    const DISCRIMINATOR: u8;
}
