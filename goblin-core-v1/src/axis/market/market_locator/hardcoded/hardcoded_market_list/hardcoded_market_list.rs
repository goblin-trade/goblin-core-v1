use crate::{
    axis::market::{token_pair::TokenPair, Hardcoded, MarketReadables},
    axis_helpers::MarketSpec,
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
