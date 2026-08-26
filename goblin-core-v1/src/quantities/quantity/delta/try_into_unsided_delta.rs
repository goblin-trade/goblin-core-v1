use crate::{
    axis::leg::{leg_quantities::LegQuantities, Base, Quote},
    goblin_error::GoblinError,
    quantities::{BaseDim, Dim, Exp, Quantity, QuoteDim, Unsided, Z0},
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
    for Quantity<Dim<BaseDim<L, U, A>, QuoteDim<Z0, Z0, Z0>, Z0>, u64>
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
    for Quantity<Dim<BaseDim<Z0, Z0, Z0>, QuoteDim<L, U, A>, Z0>, u64>
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
