use core::marker::PhantomData;

use crate::{
    axis::market::market_locator::hardcoded::HardcodedMarketList, axis_helpers::MarketSpec,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct MarketIndex<MS>
where
    MS: MarketSpec,
    MS::Pair: HardcodedMarketList,
{
    pub inner: usize,
    _marker: PhantomData<MS>,
}

impl<MS> MarketIndex<MS>
where
    MS: MarketSpec,
    MS::Pair: HardcodedMarketList,
{
    pub const MAX: Self =
        Self::new(<MS::Pair as HardcodedMarketList>::HARDCODED_MARKET_LIST.len() - 1);

    pub const fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
