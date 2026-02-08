use core::marker::PhantomData;

use crate::types::{Marker, Store};

pub struct TupleV2<T0, T1, K>(T0, T1, PhantomData<K>);

impl<T0, T1, K> Store<TupleV2<T0, T1, K>> for Marker<K, 0>
where
    T0: Clone + Copy,
{
    type Result = T0;

    fn get(store: &TupleV2<T0, T1, K>) -> Self::Result {
        store.0
    }

    fn get_ref(store: &TupleV2<T0, T1, K>) -> &Self::Result {
        &store.0
    }

    fn get_mut(store: &mut TupleV2<T0, T1, K>) -> &mut Self::Result {
        &mut store.0
    }
}

impl<T0, T1, K> Store<TupleV2<T0, T1, K>> for Marker<K, 1>
where
    T1: Clone + Copy,
{
    type Result = T1;

    fn get(store: &TupleV2<T0, T1, K>) -> Self::Result {
        store.1
    }

    fn get_ref(store: &TupleV2<T0, T1, K>) -> &Self::Result {
        &store.1
    }

    fn get_mut(store: &mut TupleV2<T0, T1, K>) -> &mut Self::Result {
        &mut store.1
    }
}
