use core::marker::PhantomData;

use crate::{
    axis::leg::{
        leg_coordinates::LegCoordinates, leg_iterator::LegIterator, leg_matcher::LegMatcher,
    },
    matching::bitmap::{inner_coordinates::InnerCoordinates, Coordinate},
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct CompactCoordinates<In>
where
    In: LegIterator,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> CompactCoordinates<In>
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

impl<In> Coordinate for CompactCoordinates<In>
where
    In: LegCoordinates + LegIterator,
{
    type Inner = u8;
    const MIN: Self = Self::new(0);
    const MAX: Self = Self::new(u8::MAX);

    fn inner(self) -> Self::Inner {
        self.inner
    }

    fn iter(self) -> impl Iterator<Item = Self> {
        In::coordinates_iter(self)
    }

    fn closer_to_centre(self, other: Self) -> bool {
        In::closer_to_centre(self, other)
    }
}

impl<In> From<InnerCoordinates<In>> for CompactCoordinates<In>
where
    In: LegMatcher,
{
    fn from(value: InnerCoordinates<In>) -> Self {
        CompactCoordinates::new(value.row.inner * 8 + value.column.inner)
    }
}
