use crate::{
    quantities::{Exp, Quantity},
    types::Tuple,
};

/// Widen a tuple of 32 bit quantities to their 64 bit equivalents.
///
/// 32 bit quantities are used in wire headers to reduce encoded size and are
/// widened before internal math. The widening is infallible, so this mirrors
/// `From<Quantity<E, u32>> for Quantity<E, u64>` rather than `TryFrom`.
impl<E0, E1, K> From<&Tuple<Quantity<E0, u32>, Quantity<E1, u32>, K>>
    for Tuple<Quantity<E0, u64>, Quantity<E1, u64>, K>
where
    E0: Exp,
    E1: Exp,
{
    fn from(value: &Tuple<Quantity<E0, u32>, Quantity<E1, u32>, K>) -> Self {
        let t0 = Quantity::<E0, u64>::from(value.0);
        let t1 = Quantity::<E1, u64>::from(value.1);
        Tuple::new(t0, t1)
    }
}
