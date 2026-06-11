use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Quote},
    quantities::{BaseDim, Dim, Exp, Quantity, QuoteDim, Unsided, Z0},
};

/// Trait to unside quantity
/// We cannot use From for base and quote forms because compiler cannot prove
/// there is no overlap.
pub trait UnsideQuantity<S: LegQuantities, L: Exp, U: Exp, A: Exp> {
    fn unsided(self) -> Unsided<L, U, A>;
}

/// Base → Unsided
impl<L: Exp, U: Exp, A: Exp> UnsideQuantity<Base, L, U, A>
    for Quantity<Dim<BaseDim<L, U, A>, QuoteDim<Z0, Z0, Z0>, Z0>>
{
    fn unsided(self) -> Unsided<L, U, A> {
        Quantity::new(self.inner)
    }
}

/// Quote → Unsided
impl<L: Exp, U: Exp, A: Exp> UnsideQuantity<Quote, L, U, A>
    for Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<L, U, A>, Z0>>
{
    fn unsided(self) -> Unsided<L, U, A> {
        Quantity::new(self.inner)
    }
}
