///! We have 2 market variants
///!
///! * Hardcoded market- has hardcoded tokens
///! * Dynamic market- has dynamic tokens that can be either hardcoded or custom
///!
///! However since we use generics, all combinations must be implented. Even hardcoded
///! markets with custom tokens.
use crate::{market::MarketLocator, token::TokenMarker};

pub trait MarketVariant: Sized {
    /// Discriminator used to hash the market key
    const DISCRIMINATOR: u8;

    /// The intermediate representation used to locate a market.
    ///
    /// - Hardcoded: A market index for lookup
    /// - Dynamic: The complete MarketAndKey (acts as its own locator)
    type MarketLocator<B: TokenMarker, Q: TokenMarker>: MarketLocator<Self, B, Q>;
}
