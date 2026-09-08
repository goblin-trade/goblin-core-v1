use super::{HardcodedMarketList, MarketIndex};
use core::ops::Index;

use crate::{axis_helpers::TokenPair, market::MarketReadables};

impl<TP> Index<MarketIndex<TP>> for &'static [MarketReadables<TP>]
where
    TP: TokenPair + HardcodedMarketList,
{
    type Output = MarketReadables<TP>;

    fn index(&self, index: MarketIndex<TP>) -> &Self::Output {
        self.get(index.inner).unwrap()
    }
}
