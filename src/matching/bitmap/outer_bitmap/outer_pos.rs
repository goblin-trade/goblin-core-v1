use core::marker::PhantomData;

use crate::{
    axis::leg::leg_coordinates::LegCoordinates, matching::bitmap::Coordinate, quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OuterPos<In>
where
    In: LegCoordinates,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> OuterPos<In>
where
    In: LegCoordinates,
{
    pub const fn new(inner: u8) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<In> Coordinate for OuterPos<In>
where
    In: LegCoordinates,
{
    const MIN: Self = OuterPos::new(0);
    const MAX: Self = OuterPos::new(255);

    fn iter(self) -> impl Iterator<Item = Self> {
        In::outer_pos_iter(self)
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
    }
}

impl<In> From<Ticks> for OuterPos<In>
where
    In: LegCoordinates,
{
    fn from(value: Ticks) -> Self {
        Self::new(((value.inner / 32) % 256) as u8)
    }
}
