use crate::{
    axis::market::Hardcoded,
    axis_helpers::MarketSpec,
    market::{MarketReadables, TokenPair},
};

/// Trait to store hardcoded market lists for each B, Q combination.
/// Used with MarketLocator trait
pub trait HardcodedMarketList
where
    Self: TokenPair,
    (Hardcoded, Self): MarketSpec,
{
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<(Hardcoded, Self)>];
}
