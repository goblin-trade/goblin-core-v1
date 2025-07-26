use crate::{
    markets::IndexedMarket,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    tokens::{NamedToken, TokenIndex},
};

pub const HARDCODED_MARKETS: [IndexedMarket; 2] = [
    IndexedMarket::new_unchecked(
        TokenIndex::ETH,
        NamedToken::USDCoin.index(),
        BaseLotsPerBaseUnit(100),
        QuoteLotsPerQuoteUnit(1000),
        QuoteLotsPerBaseUnitPerTick(1),
    ),
    IndexedMarket::new_unchecked(
        NamedToken::WrappedBTC.index(),
        NamedToken::USDCoin.index(),
        BaseLotsPerBaseUnit(100),
        QuoteLotsPerQuoteUnit(1000),
        QuoteLotsPerBaseUnitPerTick(1),
    ),
];
