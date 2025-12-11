use core::marker::PhantomData;

/// Two field storage for marker type K. K is a tuple of like marker structs
/// (Base, Quote). The markers are used with `TupleReader` trait to
/// zero-cost access the elements at compile time.
///
/// # Example usage
///
/// type MakerDeltaPair = Tuple<MakerDelta<Base>, MakerDelta<Quote>, (Base, Quote)>;
///
#[derive(Clone, Copy, Default)]
pub struct Tuple<T0, T1, K>(pub T0, pub T1, PhantomData<K>)
where
    T0: Clone + Copy,
    T1: Clone + Copy;

impl<T0, T1, K> Tuple<T0, T1, K>
where
    T0: Clone + Copy,
    T1: Clone + Copy,
{
    pub const fn new(t0: T0, t1: T1) -> Self {
        Self(t0, t1, PhantomData)
    }
}

pub trait TupleReader<T0, T1, K>
where
    T0: Clone + Copy,
    T1: Clone + Copy,
{
    type Result;
    fn get(tuple: &Tuple<T0, T1, K>) -> Self::Result;
    fn get_leg(tuple: &Tuple<T0, T1, K>) -> &Self::Result;
    fn get_leg_mut(tuple: &mut Tuple<T0, T1, K>) -> &mut Self::Result;
}
