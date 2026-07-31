use crate::axis::market::{market_locator::hardcoded::HardcodedMarkets, token_pair::TokenPair};
use core::marker::PhantomData;

/// Index to read a hardcoded market from the static list.
/// This index is validated for bounds during decoding
#[derive(Clone, Copy)]
pub struct HardcodedMarketIndex<TP>
where
    TP: TokenPair,
    Self: HardcodedMarkets<TP>,
{
    pub inner: usize,
    _marker: PhantomData<TP>,
}

impl<TP> HardcodedMarketIndex<TP>
where
    TP: TokenPair,
    Self: HardcodedMarkets<TP>,
{
    pub fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
