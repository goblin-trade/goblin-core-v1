use crate::{
    markets::Market,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick},
    tokens::HARDCODED_TOKENS,
};

pub const HARDCODED_MARKETS: [Market; 1] = [Market::new_unchecked(
    HARDCODED_TOKENS[0],
    HARDCODED_TOKENS[1],
    BaseLotsPerBaseUnit(100),
    QuoteLotsPerBaseUnit(1000),
    QuoteLotsPerBaseUnitPerTick(1),
)];
