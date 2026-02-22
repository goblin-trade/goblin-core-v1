use core::marker::PhantomData;

use crate::{
    axis::leg::leg_coordinates::LegCoordinates,
    matching::bitmap::{compact_coordinates::CompactCoordinates, Coordinate},
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Row<In>
where
    In: LegCoordinates,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> Row<In>
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

impl<In> Coordinate for Row<In>
where
    In: LegCoordinates,
{
    const MIN: Self = Row::new(0);
    const MAX: Self = Row::new(31);

    fn iter(self) -> impl Iterator<Item = Self> {
        In::row_iter(self)
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
    }
}

impl<In> From<CompactCoordinates<In>> for Row<In>
where
    In: LegCoordinates,
{
    fn from(value: CompactCoordinates<In>) -> Self {
        Row::new(value.inner / 8)
    }
}

impl<In> From<Ticks> for Row<In>
where
    In: LegCoordinates,
{
    fn from(value: Ticks) -> Self {
        Self::new((value.inner % 32) as u8)
    }
}
