use crate::{
    markets::IndexedMarket,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::TokenIndex,
};

pub const HARDCODED_MARKETS: [IndexedMarket; 1] = [IndexedMarket::new_unchecked(
    TokenIndex(0),
    TokenIndex(1),
    BaseLotsPerBaseUnit::new(100),
    QuoteLotsPerQuoteUnit::new(1000),
    QuoteLotsPerBaseUnitPerTick::new(1),
)];
