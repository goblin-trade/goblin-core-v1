use core::marker::PhantomData;

use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, range::Range, Coordinate},
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
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

    pub fn get_start(self, range: Range<OuterBitmapIndex<In>>) -> Self {
        if range.on_limit() {
            self
        } else {
            In::start_value()
        }
    }

    pub fn get_limit(self, range: Range<OuterBitmapIndex<In>>) -> Self {
        if range.on_limit() {
            self
        } else {
            In::end_value()
        }
    }
}

impl<In> Coordinate for OuterPos<In>
where
    In: LegMatcher,
{
    type Inner = u8;
    const MIN: Self = OuterPos::new(0);
    const MAX: Self = OuterPos::new(255);

    fn inner(self) -> Self::Inner {
        self.inner
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
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
