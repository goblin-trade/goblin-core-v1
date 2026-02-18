use core::marker::PhantomData;

use crate::axis::leg::leg_coordinates::LegCoordinates;

#[derive(Clone, Copy)]
pub struct InnerPos<In>
where
    In: LegCoordinates,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> InnerPos<In>
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
