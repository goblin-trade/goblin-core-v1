use core::marker::PhantomData;
use core::u64;

use crate::quantities::Exp;

//
// Quantity type: value + Dim
//
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Quantity<E: Exp> {
    pub inner: u64,
    _marker: PhantomData<E>,
}

impl<E: Exp> Quantity<E> {
    pub const fn new(value: u64) -> Self {
        Self {
            inner: value,
            _marker: PhantomData,
        }
    }
}

impl<E: Exp> From<u64> for Quantity<E> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
