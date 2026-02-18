use core::marker::PhantomData;

use crate::{axis::leg::leg_coordinates::LegCoordinates, quantities::Ticks};

#[derive(Clone, Copy)]
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

    pub fn iter(self) -> impl Iterator<Item = Self> {
        In::outer_bitmap_index_iter(self)
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
