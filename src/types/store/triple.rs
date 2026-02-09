use core::marker::PhantomData;

use crate::types::{Marker, StoreReader};

#[derive(Default, Clone, Copy)]
pub struct Triple<T0, T1, T2, K>(T0, T1, T2, PhantomData<K>);

impl<T0, T1, T2, K> Triple<T0, T1, T2, K> {
    pub const fn new(t0: T0, t1: T1, t2: T2) -> Self {
        Self(t0, t1, t2, PhantomData)
    }
}

impl<T0, T1, T2, K> StoreReader<Triple<T0, T1, T2, K>> for Marker<K, 0>
where
    T0: Clone + Copy,
{
    type Result = T0;

    fn get(store: &Triple<T0, T1, T2, K>) -> Self::Result {
        store.0
    }

    fn get_leg(store: &Triple<T0, T1, T2, K>) -> &Self::Result {
        &store.0
    }

    fn get_leg_mut(store: &mut Triple<T0, T1, T2, K>) -> &mut Self::Result {
        &mut store.0
    }
}

impl<T0, T1, T2, K> StoreReader<Triple<T0, T1, T2, K>> for Marker<K, 1>
where
    T1: Clone + Copy,
{
    type Result = T1;

    fn get(store: &Triple<T0, T1, T2, K>) -> Self::Result {
        store.1
    }

    fn get_leg(store: &Triple<T0, T1, T2, K>) -> &Self::Result {
        &store.1
    }

    fn get_leg_mut(store: &mut Triple<T0, T1, T2, K>) -> &mut Self::Result {
        &mut store.1
    }
}

impl<T0, T1, T2, K> StoreReader<Triple<T0, T1, T2, K>> for Marker<K, 2>
where
    T2: Clone + Copy,
{
    type Result = T2;

    fn get(store: &Triple<T0, T1, T2, K>) -> Self::Result {
        store.2
    }

    fn get_leg(store: &Triple<T0, T1, T2, K>) -> &Self::Result {
        &store.2
    }

    fn get_leg_mut(store: &mut Triple<T0, T1, T2, K>) -> &mut Self::Result {
        &mut store.2
    }
}
