use core::ops::Index;

use crate::{
    axis::market::{
        market_locator::{hardcoded::HardcodedMarketList, MarketIndex},
        MarketReadables,
    },
    axis_helpers::MarketSpec,
};

impl<MS> Index<MarketIndex<MS>> for &'static [MarketReadables<MS>]
where
    MS: MarketSpec,
    MS::Pair: HardcodedMarketList,
{
    type Output = MarketReadables<MS>;

    fn index(&self, index: MarketIndex<MS>) -> &Self::Output {
        self.get(index.inner).unwrap()
    }
}
