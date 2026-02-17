use core::marker::PhantomData;

use crate::axis::leg::leg_matcher::LegMatcher;

#[derive(Clone, Copy)]
pub struct InnerPos<In>
where
    In: LegMatcher,
{
    pub inner: u8,
    _marker: PhantomData<In>,
}

impl<In> InnerPos<In>
where
    In: LegMatcher,
{
    pub const fn new(inner: u8) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}
