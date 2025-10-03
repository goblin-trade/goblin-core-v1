use crate::{
    markets::{CommonMarket, HardcodedMarket, IndexedMarket},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::{DynamicTokenIndex, HardcodedIndex, TokenPair},
    types::Pair,
};

// Problem- if we use token_pair: TokenPair<T>,
// then a common list handles the 3 token pair variants
// Should we have 3 different hardcoded lists, i.e. 3 distinct hardcoded market types
// for each variant of TokenPair?
// This means we have 3 types of MarketIndices now.
pub const HARDCODED_MARKETS: [HardcodedMarket; 1] = [HardcodedMarket {
    common_market: CommonMarket {
        token_pair: TokenPair::ERC20BaseETHQuote(HardcodedIndex(0)),
        lot_size_pair: Pair {
            base: BaseLotsPerBaseUnit::new(100),
            quote: QuoteLotsPerQuoteUnit::new(1000),
        },
        tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
    },
    keccak_hash: [0u8; 32],
    // token_index_pair: Pair {
    //     base: DynamicTokenIndex(0),
    //     quote: DynamicTokenIndex(1),
    // },
    // lot_size_pair: Pair {
    //     base: BaseLotsPerBaseUnit::new(100),
    //     quote: QuoteLotsPerQuoteUnit::new(1000),
    // },
    // tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
}];
