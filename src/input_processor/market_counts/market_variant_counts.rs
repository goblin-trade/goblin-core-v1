use core::marker::PhantomData;

use crate::market::MarketVariant;

/// Market counts per variant
#[derive(Clone, Copy)]
pub struct MarketVariantCounts<M: MarketVariant> {
    inner: [u8; 3],
    _marker: PhantomData<M>,
}

impl<M: MarketVariant> MarketVariantCounts<M> {
    pub fn new(inner: [u8; 3]) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn process(&self) {}
}
