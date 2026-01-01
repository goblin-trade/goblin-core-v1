///! Unsided atoms are used in the global delta where there is no notion of side
use crate::{
    quantities::{BaseDim, Dim, Exp, Quantity, QuoteDim, SidedDim, P1, Z0},
    types::{Base, LegQuantities, Quote},
};

pub type Unsided<L, U, A> = Quantity<SidedDim<L, U, A>>;
pub type UnsidedAtoms = Unsided<Z0, Z0, P1>;

pub trait AsUnsided<S: LegQuantities, L: Exp, U: Exp, A: Exp> {
    fn unsided(self) -> Unsided<L, U, A>;
}

/// Base → Unsided
impl<L: Exp, U: Exp, A: Exp> AsUnsided<Base, L, U, A>
    for Quantity<Dim<BaseDim<L, U, A>, QuoteDim<Z0, Z0, Z0>, Z0>>
{
    fn unsided(self) -> Unsided<L, U, A> {
        Quantity::new(self.inner)
    }
}

/// Quote → Unsided
impl<L: Exp, U: Exp, A: Exp> AsUnsided<Quote, L, U, A>
    for Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<L, U, A>, Z0>>
{
    fn unsided(self) -> Unsided<L, U, A> {
        Quantity::new(self.inner)
    }
}
