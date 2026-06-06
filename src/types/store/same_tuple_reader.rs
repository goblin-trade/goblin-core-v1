use crate::types::same_tuple::SameTuple;

pub trait SameTupleReader<T, K> {
    fn get(store: &SameTuple<T, K>) -> T;
    fn get_leg(store: &SameTuple<T, K>) -> &T;
    fn get_leg_mut(store: &mut SameTuple<T, K>) -> &mut T;
}
