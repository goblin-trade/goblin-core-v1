use core::marker::PhantomData;

use crate::{axis::leg::leg_matcher::LegMatcher, quantities::Ticks};

#[derive(Clone, Copy)]
pub struct OuterBitmapIndex<In>
where
    In: LegMatcher,
{
    pub inner: u64,
    _marker: PhantomData<In>,
}

impl<In> OuterBitmapIndex<In>
where
    In: LegMatcher,
{
    pub const fn new(inner: u64) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<In> From<Ticks> for OuterBitmapIndex<In>
where
    In: LegMatcher,
{
    fn from(value: Ticks) -> Self {
        Self::new(value.inner / (256 * 32))
    }
}
