use crate::axis::market::token_pair::TokenPair;
use core::marker::PhantomData;

/// Index to read a hardcoded market from the static list.
/// This index has NOT been validated for bounds. Bound check happens when reading
/// the market.
#[derive(Clone, Copy)]
pub struct HardcodedMarketIndex<TP: TokenPair> {
    pub inner: usize,
    _marker: PhantomData<TP>,
}

impl<TP: TokenPair> HardcodedMarketIndex<TP> {
    pub fn new(inner: usize) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
