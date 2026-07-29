use crate::axis::market::{
    market_spec::MarketSpec, token_pair::TokenPair, Hardcoded, MarketReadables,
};

/// Trait to store hardcoded market lists for each B, Q combination.
/// Used with MarketLocator trait
pub trait HardcodedMarkets<TP: TokenPair>
where
    (Hardcoded, TP): MarketSpec,
{
    const HARDCODED_MARKETS: &'static [MarketReadables<(Hardcoded, TP)>];
}
