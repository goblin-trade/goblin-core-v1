use core::marker::PhantomData;

use crate::{
    axis::market::market_locator::hardcoded::HardcodedMarketList, axis_helpers::TokenPair,
};

/// Index for hardcoded market
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    pub inner: usize,
    _marker: PhantomData<TP>,
}

impl<TP> MarketIndex<TP>
where
    TP: TokenPair + HardcodedMarketList,
{
    pub const fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
