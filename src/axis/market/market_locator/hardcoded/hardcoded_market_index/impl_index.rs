use core::ops::Index;

use crate::axis::market::{
    market_locator::hardcoded::{HardcodedMarketIndex, HardcodedMarkets},
    Hardcoded, MarketReadables, TokenPair,
};

impl<TP> Index<HardcodedMarketIndex<TP>> for &'static [MarketReadables<(Hardcoded, TP)>]
where
    TP: TokenPair,
    HardcodedMarketIndex<TP>: HardcodedMarkets<TP>,
{
    type Output = MarketReadables<(Hardcoded, TP)>;

    fn index(&self, index: HardcodedMarketIndex<TP>) -> &Self::Output {
        self.get(index.inner).unwrap()
    }
}
