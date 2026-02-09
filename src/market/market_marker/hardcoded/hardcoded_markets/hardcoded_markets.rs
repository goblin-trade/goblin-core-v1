use crate::{market::MarketAndKey, token::TokenMarker, types::Hardcoded};

/// Trait to store hardcoded market lists for each B, Q combination.
/// Used with MarketLocator trait
pub trait HardcodedMarkets<B: TokenMarker, Q: TokenMarker> {
    const HARDCODED_MARKETS: &'static [MarketAndKey<Hardcoded, B, Q>];
}
