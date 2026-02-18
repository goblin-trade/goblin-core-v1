use core::marker::PhantomData;

use crate::{axis::leg::leg_coordinates::LegCoordinates, quantities::Ticks};

#[derive(Clone, Copy)]
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

impl<In> From<Ticks> for OuterPos<In>
where
    In: LegCoordinates,
{
    fn from(value: Ticks) -> Self {
        Self::new(((value.inner / 32) % 256) as u8)
    }
}
