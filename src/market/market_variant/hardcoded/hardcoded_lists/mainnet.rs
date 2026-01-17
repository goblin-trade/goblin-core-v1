use crate::{
    markets::IndexedMarketV2,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::{NamedToken, TokenIndex},
};

pub const HARDCODED_MARKETS: [IndexedMarketV2; 1] = [IndexedMarketV2 {
    base: MarketLeg {
        token_index: TokenIndex::ETH,
        lot_size: BaseLotsPerBaseUnit(100),
    },
    quote: MarketLeg {
        token_index: NamedToken::USDCoin.index(),
        lot_size: QuoteLotsPerQuoteUnit(1000),
    },
    tick_size: QuoteLotsPerBaseUnitPerTick::new(1),
}];

// pub const HARDCODED_MARKETS: [IndexedMarket; 2] = [
//     IndexedMarket::new_unchecked(
//         TokenIndex::ETH,
//         NamedToken::USDCoin.index(),
//         BaseLotsPerBaseUnit(100),
//         QuoteLotsPerQuoteUnit(1000),
//         QuoteLotsPerBaseUnitPerTick(1),
//     ),
//     IndexedMarket::new_unchecked(
//         NamedToken::WrappedBTC.index(),
//         NamedToken::USDCoin.index(),
//         BaseLotsPerBaseUnit(100),
//         QuoteLotsPerQuoteUnit(1000),
//         QuoteLotsPerBaseUnitPerTick(1),
//     ),
// ];
