use crate::{
    markets::{IndexedMarketV2, MarketLeg},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::TokenIndex,
};

pub const HARDCODED_MARKETS: [IndexedMarketV2; 1] = [IndexedMarketV2 {
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
