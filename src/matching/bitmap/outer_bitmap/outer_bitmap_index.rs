use core::marker::PhantomData;

use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, leg_iterator::LegIterator},
    matching::bitmap::Coordinate,
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OuterBitmapIndex<In>
where
    In: LegIterator,
{
    pub inner: u64,
    _marker: PhantomData<In>,
}

impl<In> OuterBitmapIndex<In>
where
    In: LegIterator,
{
    pub const fn new(inner: u64) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<In> Coordinate for OuterBitmapIndex<In>
where
    In: LegCoordinates + LegIterator,
{
    type Inner = u64;
    const MIN: Self = OuterBitmapIndex::new(0);
    const MAX: Self = OuterBitmapIndex::new(u64::MAX);

    fn inner(self) -> Self::Inner {
        self.inner
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
    }
}

impl<In> From<Ticks> for OuterBitmapIndex<In>
where
    In: LegIterator,
{
    fn from(value: Ticks) -> Self {
        Self::new(value.inner / (256 * 32))
    }
}
