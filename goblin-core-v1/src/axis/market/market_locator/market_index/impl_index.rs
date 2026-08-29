use core::ops::Index;

use crate::{
    axis::market::market_locator::{hardcoded::HardcodedMarketList, MarketIndex},
    axis_helpers::TokenPair,
    market::MarketReadables,
};

impl<TP> Index<MarketIndex<TP>> for &'static [MarketReadables<TP>]
where
    TP: TokenPair + HardcodedMarketList,
{
    type Output = MarketReadables<TP>;

    fn index(&self, index: MarketIndex<TP>) -> &Self::Output {
        self.get(index.inner).unwrap()
    }
}
