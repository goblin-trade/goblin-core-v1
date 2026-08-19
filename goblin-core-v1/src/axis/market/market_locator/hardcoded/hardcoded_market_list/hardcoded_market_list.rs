use crate::{axis::market::Hardcoded, axis_helpers::MarketSpec, market::MarketReadables};

/// Trait to store hardcoded market lists for each B, Q combination.
/// Used with MarketLocator trait
pub trait HardcodedMarketList: 'static
where
    (Hardcoded, Self): MarketSpec,
{
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<(Hardcoded, Self)>];
}
