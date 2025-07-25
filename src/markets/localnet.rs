use crate::{
    markets::IndexedMarket,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::TokenIndex,
};

pub const HARDCODED_MARKETS: [IndexedMarket; 1] = [IndexedMarket::new_unchecked(
    TokenIndex(0),
    TokenIndex(1),
    BaseLotsPerBaseUnit(100),
    QuoteLotsPerQuoteUnit(1000),
    QuoteLotsPerBaseUnitPerTick(1),
)];
