use crate::{
    markets::{IndexedMarket, MarketLeg},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::TokenIndex,
};

pub const HARDCODED_MARKETS: [IndexedMarket; 1] = [IndexedMarket {
    base: MarketLeg {
        token_index: TokenIndex(0),
        lot_size: BaseLotsPerBaseUnit::new(100),
    },
    quote: MarketLeg {
        token_index: TokenIndex(1),
        lot_size: QuoteLotsPerQuoteUnit::new(1000),
    },
    tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
}];

// pub const HARDCODED_MARKETS: [IndexedMarket; 1] = [IndexedMarket::new_unchecked(
//     TokenIndex(0),
//     TokenIndex(1),
//     BaseLotsPerBaseUnit::new(100),
//     QuoteLotsPerQuoteUnit::new(1000),
//     QuoteLotsPerBaseUnitPerTick::new(1),
// )];
