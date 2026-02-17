use core::marker::PhantomData;

use crate::{axis::leg::leg_matcher::LegMatcher, quantities::Ticks};

#[derive(Clone, Copy)]
pub struct OuterPos<In>
where
    In: LegMatcher,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> OuterPos<In>
where
    In: LegMatcher,
{
    pub const fn new(inner: u8) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<In> From<Ticks> for OuterPos<In>
where
    In: LegMatcher,
{
    fn from(value: Ticks) -> Self {
        Self::new(((value.inner / 32) % 256) as u8)
    }
}
