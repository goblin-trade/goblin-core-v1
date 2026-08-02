use core::marker::PhantomData;

#[derive(Default, Clone, Copy)]
pub struct Triple<T0, T1, T2, K>(pub T0, pub T1, pub T2, PhantomData<K>);

impl<T0, T1, T2, K> Triple<T0, T1, T2, K> {
    pub const fn new(t0: T0, t1: T1, t2: T2) -> Self {
        Self(t0, t1, t2, PhantomData)
    }
}
