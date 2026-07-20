use crate::types::{triple::triple::Triple, Marker, RefReader};

impl<'a, T0, T1, T2, K> RefReader<'a, Triple<T0, T1, T2, K>> for Marker<K, 0>
where
    T0: Clone + Copy,
{
    type Result = T0;

    fn get_with_lifetime(store: &'a Triple<T0, T1, T2, K>) -> Self::Result {
        store.0
    }
}

impl<'a, T0, T1, T2, K> RefReader<'a, Triple<T0, T1, T2, K>> for Marker<K, 1>
where
    T1: Clone + Copy,
{
    type Result = T1;

    fn get_with_lifetime(store: &'a Triple<T0, T1, T2, K>) -> Self::Result {
        store.1
    }
}

impl<'a, T0, T1, T2, K> RefReader<'a, Triple<T0, T1, T2, K>> for Marker<K, 2>
where
    T2: Clone + Copy,
{
    type Result = T2;

    fn get_with_lifetime(store: &'a Triple<T0, T1, T2, K>) -> Self::Result {
        store.2
    }
}
