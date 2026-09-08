mod impl_checked_ops;
mod impl_const_default;
mod impl_div;
mod impl_fixed_decode;
mod impl_mul;
mod impl_store_reader;
mod impl_try_from;
mod impl_unside_quantity;

use core::marker::PhantomData;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Tuple<T0, T1, K>(pub T0, pub T1, PhantomData<K>);

impl<T0, T1, K> Tuple<T0, T1, K> {
    pub const fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}
