use crate::{
    axis::leg::{Base, Quote, leg_quantities::LegQuantities},
    quantities::{Dim, Exp, Quantity, QuantityOps, SidedDim, Unsided, Z0},
};

/// Trait to unside quantity
/// We cannot use From for base and quote forms because compiler cannot prove
/// there is no overlap.
pub trait UnsideQuantity<S>
where
    S: LegQuantities,
{
    type Output;

    fn unsided(self) -> Self::Output;
}

/// Base → Unsided
impl<L, U, A, I> UnsideQuantity<Base>
    for Quantity<Dim<SidedDim<Base, L, U, A>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, I>
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
    for Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, L, U, A>, Z0>, I>
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
