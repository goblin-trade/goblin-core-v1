use core::marker::PhantomData;

use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, leg_iterator::LegIterator},
    matching::bitmap::{inner_pos::InnerPos, Coordinate},
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Row<In>
where
    In: LegIterator,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> Row<In>
where
    In: LegIterator,
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
    In: LegCoordinates + LegIterator,
{
    type Inner = u8;
    const MIN: Self = Row::new(0);
    const MAX: Self = Row::new(31);

    fn inner(self) -> Self::Inner {
        self.inner
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
    }
}

impl<In> From<InnerPos<In>> for Row<In>
where
    In: LegIterator,
{
    fn from(value: InnerPos<In>) -> Self {
        Row::new(value.inner / 8)
    }
}

impl<In> From<Ticks> for Row<In>
where
    In: LegIterator,
{
    fn from(value: Ticks) -> Self {
        Self::new((value.inner % 32) as u8)
    }
}
