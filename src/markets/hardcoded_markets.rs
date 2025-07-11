use crate::{
    markets::Market,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick},
    tokens::HARDCODED_TOKENS,
};

pub const HARDCODED_MARKETS: [Market; 1] = [Market::new_unchecked(
    HARDCODED_TOKENS[0].address,
    HARDCODED_TOKENS[1].address,
    BaseLotsPerBaseUnit(100),
    QuoteLotsPerBaseUnit(1000),
    QuoteLotsPerBaseUnitPerTick(1),
)];
