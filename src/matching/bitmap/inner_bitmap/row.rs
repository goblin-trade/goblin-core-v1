use core::marker::PhantomData;

use crate::{
    axis::leg::leg_matcher::LegMatcher, matching::bitmap::inner_pos::InnerPos, quantities::Ticks,
};

pub struct Row<In>
where
    In: LegMatcher,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> Row<In>
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

impl<In> From<InnerPos<In>> for Row<In>
where
    In: LegMatcher,
{
    fn from(value: InnerPos<In>) -> Self {
        Row::new(value.inner / 8)
    }
}

impl<In> From<Ticks> for Row<In>
where
    In: LegMatcher,
{
    fn from(value: Ticks) -> Self {
        Self::new((value.inner % 32) as u8)
    }
}
