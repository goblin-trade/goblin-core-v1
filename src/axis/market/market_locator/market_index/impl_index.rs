use core::ops::Index;

use crate::axis::market::{
    market_locator::{hardcoded::HardcodedMarkets, MarketIndex},
    market_spec::MarketSpec,
    MarketReadables,
};

impl<MS> Index<MarketIndex<MS>> for &'static [MarketReadables<MS>]
where
    MS: MarketSpec,
    MarketIndex<MS>: HardcodedMarkets<MS::Pair>,
{
    type Output = MarketReadables<MS>;

    fn index(&self, index: MarketIndex<MS>) -> &Self::Output {
        self.get(index.inner).unwrap()
    }
}
