use core::marker::PhantomData;

use crate::axis::market::{market_locator::hardcoded::HardcodedMarkets, market_spec::MarketSpec};

#[derive(Clone, Copy)]
pub struct MarketIndex<MS>
where
    MS: MarketSpec,
    Self: HardcodedMarkets<MS::Pair>,
{
    pub inner: usize,
    _marker: PhantomData<MS>,
}

impl<MS> MarketIndex<MS>
where
    MS: MarketSpec,
    Self: HardcodedMarkets<MS::Pair>,
{
    pub fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
