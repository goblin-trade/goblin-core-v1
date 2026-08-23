use crate::types::{Marker, StoreReader, Tuple};

impl<T0, T1, K> StoreReader<Tuple<T0, T1, K>> for Marker<K, 0> {
    type Result = T0;

    fn get(store: &Tuple<T0, T1, K>) -> Self::Result
    where
        Self::Result: Clone + Copy,
    {
        store.0
    }

    fn get_leg(store: &Tuple<T0, T1, K>) -> &Self::Result {
        &store.0
    }

    fn get_leg_mut(store: &mut Tuple<T0, T1, K>) -> &mut Self::Result {
        &mut store.0
    }
}

impl<T0, T1, K> StoreReader<Tuple<T0, T1, K>> for Marker<K, 1> {
    type Result = T1;

    fn get(store: &Tuple<T0, T1, K>) -> Self::Result
    where
        Self::Result: Clone + Copy,
    {
        store.1
    }

    fn get_leg(store: &Tuple<T0, T1, K>) -> &Self::Result {
        &store.1
    }

    fn get_leg_mut(store: &mut Tuple<T0, T1, K>) -> &mut Self::Result {
        &mut store.1
    }
}
