use core::marker::PhantomData;

/// Three field storage for marker type K. K is a tuple of marker structs
/// (ETH, HardcodedERC20, CustomERC20).
///
/// The markers are used with `TripleReader` to provide zero-cost,
/// compile-time access to individual legs.
///
/// # Example usage
///
/// type TokenTriple = Triple<
///     Balance<ETH>,
///     Balance<HardcodedERC20>,
///     Balance<CustomERC20>,
///     (ETH, HardcodedERC20, CustomERC20),
/// >;
///
#[derive(Clone, Copy, Default)]
pub struct Triple<T0, T1, T2, K>(pub T0, pub T1, pub T2, PhantomData<K>)
where
    T0: Clone + Copy,
    T1: Clone + Copy,
    T2: Clone + Copy;

impl<T0, T1, T2, K> Triple<T0, T1, T2, K>
where
    T0: Clone + Copy,
    T1: Clone + Copy,
    T2: Clone + Copy,
{
    pub const fn new(t0: T0, t1: T1, t2: T2) -> Self {
        Self(t0, t1, t2, PhantomData)
    }
}

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
