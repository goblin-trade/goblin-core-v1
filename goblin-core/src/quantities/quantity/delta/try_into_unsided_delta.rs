use crate::{
    axis::leg::{Base, Quote, leg_quantities::LegQuantities},
    goblin_error::GoblinError,
    quantities::{Dim, Exp, Quantity, SidedDim, Unsided, Z0},
};

pub trait TryIntoUnsidedDelta<S, L, U, A>
where
    S: LegQuantities,
    L: Exp,
    U: Exp,
    A: Exp,
{
    fn try_into_unsided_delta(self) -> Result<Unsided<L, U, A, i64>, GoblinError>;
}

/// Base → Unsided i64 (from u64, fallible)
impl<L, U, A> TryIntoUnsidedDelta<Base, L, U, A>
    for Quantity<Dim<SidedDim<Base, L, U, A>, SidedDim<Quote, Z0, Z0, Z0>, Z0>, u64>
where
    L: Exp,
    U: Exp,
    A: Exp,
{
    fn try_into_unsided_delta(self) -> Result<Unsided<L, U, A, i64>, GoblinError> {
        let raw = i64::try_from(self.inner).map_err(|_| GoblinError::Overflow)?;
        let delta = Unsided::new(raw);
        Ok(delta)
    }
}

/// Quote → Unsided i64 (from u64, fallible)
impl<L, U, A> TryIntoUnsidedDelta<Quote, L, U, A>
    for Quantity<Dim<SidedDim<Base, Z0, Z0, Z0>, SidedDim<Quote, L, U, A>, Z0>, u64>
where
    L: Exp,
    U: Exp,
    A: Exp,
{
    fn try_into_unsided_delta(self) -> Result<Unsided<L, U, A, i64>, GoblinError> {
        let raw = i64::try_from(self.inner).map_err(|_| GoblinError::Overflow)?;
        let delta = Unsided::new(raw);
        Ok(delta)
    }
}
