use crate::{
    markets::IndexedMarket,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
};

pub const HARDCODED_MARKETS: [IndexedMarket; 1] = [IndexedMarket::new_unchecked(
    0,
    1,
    BaseLotsPerBaseUnit(100),
    QuoteLotsPerQuoteUnit(1000),
    QuoteLotsPerBaseUnitPerTick(1),
)];
