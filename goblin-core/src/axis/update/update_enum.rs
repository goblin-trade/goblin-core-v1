use crate::{
    quantities::{Exp, Quantity},
    settlement::ConstDefault,
    types::Tuple,
};
use goblin_macros::define_axis;

#[define_axis]
pub enum UpdateEnum {
    Increase = 0,
    Decrease = 1,
}

impl<E: Exp> From<Quantity<E, i64>> for UpdateEnum {
    fn from(value: Quantity<E, i64>) -> Self {
        if value > Quantity::DEFAULT {
            Self::Increase
        } else {
            Self::Decrease
        }
    }
}

pub type UpdatePair<T0, T1> = Tuple<T0, T1, Update>;
pub type SameUpdatePair<T> = UpdatePair<T, T>;
