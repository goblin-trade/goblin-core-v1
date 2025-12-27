use core::marker::PhantomData;
use core::u64;

use crate::quantities::Exp;

//
// Quantity type: value + Dim
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Quantity<D: Exp> {
    pub inner: u64,
    _phantom: PhantomData<D>,
}

impl<D: Exp> Quantity<D> {
    pub const fn new(value: u64) -> Self {
        Self {
            inner: value,
            _phantom: PhantomData,
        }
    }
}

impl<D: Exp> From<u64> for Quantity<D> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
