use crate::types::{Marker, SameTupleReader, Tuple};

pub type SameTuple<T, K> = Tuple<T, T, K>;

impl<T, K> SameTupleReader<T, K> for Marker<K, 0>
where
    T: Clone + Copy,
{
    fn get(store: &SameTuple<T, K>) -> T {
        store.0
    }

    fn get_leg(store: &SameTuple<T, K>) -> &T {
        &store.0
    }

    fn get_leg_mut(store: &mut SameTuple<T, K>) -> &mut T {
        &mut store.0
    }
}

impl<T, K> SameTupleReader<T, K> for Marker<K, 1>
where
    T: Clone + Copy,
{
    fn get(store: &SameTuple<T, K>) -> T {
        store.1
    }

    fn get_leg(store: &SameTuple<T, K>) -> &T {
        &store.1
    }

    fn get_leg_mut(store: &mut SameTuple<T, K>) -> &mut T {
        &mut store.1
    }
}
