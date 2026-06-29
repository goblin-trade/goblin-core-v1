use crate::{
    goblin_error::GoblinError,
    quantities::{Exp, Quantity},
};

impl<E> TryFrom<Quantity<E, u64>> for Quantity<E, i64>
where
    E: Exp,
{
    type Error = GoblinError;

    fn try_from(value: Quantity<E, u64>) -> Result<Self, Self::Error> {
        i64::try_from(value.inner)
            .map(Self::new)
            .map_err(|_| GoblinError::Overflow)
    }
}

impl<E> TryFrom<Quantity<E, i64>> for Quantity<E, u64>
where
    E: Exp,
{
    type Error = GoblinError;

    fn try_from(value: Quantity<E, i64>) -> Result<Self, Self::Error> {
        u64::try_from(value.inner)
            .map(Self::new)
            .map_err(|_| GoblinError::NegativeValue)
    }
}
