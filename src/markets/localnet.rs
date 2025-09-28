use crate::{
    markets::IndexedMarket,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::TokenIndex,
    types::Pair,
};

pub const HARDCODED_MARKETS: [IndexedMarket; 1] = [IndexedMarket {
    token_index_pair: Pair {
        base: TokenIndex(0),
        quote: TokenIndex(1),
    },
    lot_size_pair: Pair {
        base: BaseLotsPerBaseUnit::new(100),
        quote: QuoteLotsPerQuoteUnit::new(1000),
    },
    tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
}];
