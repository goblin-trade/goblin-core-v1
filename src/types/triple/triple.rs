use core::marker::PhantomData;

// TODO use in LocalDepositStore and input args

/// Three field storage for marker type K. K is a triple of marker structs like
/// ((ETH, ERC20), (ERC20, ETH), (ERC20, ERC20)).
///
/// The markers are used with `TripleReader` trait to zero-cost access the elements at compile time.
///
#[derive(Clone, Copy, Default)]
pub struct Triple<T0, T1, T2, K>(pub T0, pub T1, pub T2, PhantomData<K>)
where
    T0: Clone + Copy,
    T1: Clone + Copy,
    T2: Clone + Copy;

pub trait TripleReader<T0, T1, T2, K>
where
    T0: Clone + Copy,
    T1: Clone + Copy,
    T2: Clone + Copy,
{
    type Result;
    fn get(triple: &Triple<T0, T1, T2, K>) -> Self::Result;
    fn get_leg(triple: &Triple<T0, T1, T2, K>) -> &Self::Result;
    fn get_leg_mut(triple: &mut Triple<T0, T1, T2, K>) -> &mut Self::Result;
}
