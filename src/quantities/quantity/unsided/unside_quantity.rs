use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Quote},
    quantities::{BaseDim, Dim, Exp, Quantity, QuantityOps, QuoteDim, Unsided, Z0},
};

/// Trait to unside quantity
/// We cannot use From for base and quote forms because compiler cannot prove
/// there is no overlap.
///
/// TODO remove, replace with TryIntoUnsidedDelta
/// Unside and add delta in one step
pub trait UnsideQuantity<S>
where
    S: LegQuantities,
{
    type Output;

    fn unsided(self) -> Self::Output;
}

/// Base → Unsided
impl<L, U, A, I> UnsideQuantity<Base>
    for Quantity<Dim<BaseDim<L, U, A>, QuoteDim<Z0, Z0, Z0>, Z0>, I>
where
    L: Exp,
    U: Exp,
    A: Exp,
    I: QuantityOps,
{
    type Output = Unsided<L, U, A, I>;

    fn unsided(self) -> Self::Output {
        Quantity::new(self.inner)
    }
}

/// Quote → Unsided
impl<L, U, A, I> UnsideQuantity<Quote>
    for Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<L, U, A>, Z0>, I>
where
    L: Exp,
    U: Exp,
    A: Exp,
    I: QuantityOps,
{
    type Output = Unsided<L, U, A, I>;

    fn unsided(self) -> Self::Output {
        Quantity::new(self.inner)
    }
}
