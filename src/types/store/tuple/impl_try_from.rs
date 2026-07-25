use crate::{
    goblin_error::GoblinError,
    quantities::{Exp, Quantity},
    types::Tuple,
};

impl<E0, E1, K> TryFrom<Tuple<Quantity<E0, u64>, Quantity<E1, u64>, K>>
    for Tuple<Quantity<E0, i64>, Quantity<E1, i64>, K>
where
    E0: Exp,
    E1: Exp,
{
    type Error = GoblinError;

    fn try_from(
        value: Tuple<Quantity<E0, u64>, Quantity<E1, u64>, K>,
    ) -> Result<Self, Self::Error> {
        let t0 = Quantity::<E0, i64>::try_from(value.0)?;
        let t1 = Quantity::<E1, i64>::try_from(value.1)?;
        Ok(Tuple::new(t0, t1))
    }
}

impl<E0, E1, K> TryFrom<Tuple<Quantity<E0, i64>, Quantity<E1, i64>, K>>
    for Tuple<Quantity<E0, u64>, Quantity<E1, u64>, K>
where
    E0: Exp,
    E1: Exp,
{
    type Error = GoblinError;

    fn try_from(
        value: Tuple<Quantity<E0, i64>, Quantity<E1, i64>, K>,
    ) -> Result<Self, Self::Error> {
        let t0 = Quantity::<E0, u64>::try_from(value.0)?;
        let t1 = Quantity::<E1, u64>::try_from(value.1)?;
        Ok(Tuple::new(t0, t1))
    }
}
