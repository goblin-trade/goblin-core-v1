use core::marker::PhantomData;
use core::u64;

use crate::{
    axis::leg::leg_coordinates::LegCoordinates, matching::bitmap::Coordinate, quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OuterBitmapIndex<In>
where
    In: LegCoordinates,
{
    pub inner: u64,
    _marker: PhantomData<In>,
}

impl<In> OuterBitmapIndex<In>
where
    In: LegCoordinates,
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
    In: LegCoordinates,
{
    const MIN: Self = OuterBitmapIndex::new(0);
    const MAX: Self = OuterBitmapIndex::new(u64::MAX);

    fn iter(self) -> impl Iterator<Item = Self> {
        In::outer_bitmap_index_iter(self)
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
    }
}

impl<In> From<Ticks> for OuterBitmapIndex<In>
where
    In: LegCoordinates,
{
    fn from(value: Ticks) -> Self {
        Self::new(value.inner / (256 * 32))
    }
}
