use core::marker::PhantomData;

use crate::{
    axis::leg::{leg_iterator::LegIterator, leg_matcher::LegMatcher},
    matching::bitmap::inner_coordinates::InnerCoordinates,
};

#[derive(Clone, Copy)]
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

impl<In> From<InnerCoordinates<In>> for CompactCoordinates<In>
where
    In: LegMatcher,
{
    fn from(value: InnerCoordinates<In>) -> Self {
        CompactCoordinates::new(value.row.inner * 8 + value.column.inner)
    }
}
