use core::marker::PhantomData;

use crate::{
    axis::leg::{
        leg_coordinates::LegCoordinates, leg_matcher::LegMatcher, leg_quantities::LegQuantities,
    },
    quantities::Ticks,
};

#[derive(Clone, Copy)]
pub struct OuterBitmapIndex<In>
where
    In: LegQuantities,
{
    pub inner: u64,
    _marker: PhantomData<In>,
}

impl<In> OuterBitmapIndex<In>
where
    In: LegQuantities,
{
    pub const fn new(inner: u64) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<In> From<Ticks> for OuterBitmapIndex<In>
where
    In: LegQuantities,
{
    fn from(value: Ticks) -> Self {
        Self::new(value.inner / (256 * 32))
    }
}

impl<In> Iterator for OuterBitmapIndex<In>
where
    In: LegQuantities,
{
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        // In::nex
        None
        // let next = In::next_outer_bitmap_index(*self);

        // next
    }
}
