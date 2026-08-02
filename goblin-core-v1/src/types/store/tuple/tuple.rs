use core::marker::PhantomData;

#[derive(Default, Clone, Copy)]
pub struct Tuple<T0, T1, K>(pub T0, pub T1, PhantomData<K>);

impl<T0, T1, K> Tuple<T0, T1, K> {
    pub const fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}
