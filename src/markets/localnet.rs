use crate::{
    markets::{IndexedMarket, Market},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::HARDCODED_TOKENS,
};

// pub const HARDCODED_MARKETS: [Market; 1] = [Market::new_unchecked(
//     HARDCODED_TOKENS[0].address,
//     HARDCODED_TOKENS[1].address,
//     BaseLotsPerBaseUnit(100),
//     QuoteLotsPerQuoteUnit(1000),
//     QuoteLotsPerBaseUnitPerTick(1),
// )];

pub const HARDCODED_MARKETS: [IndexedMarket; 1] = [IndexedMarket::new_unchecked(
    0,
    1,
    BaseLotsPerBaseUnit(100),
    QuoteLotsPerQuoteUnit(1000),
    QuoteLotsPerBaseUnitPerTick(1),
)];
