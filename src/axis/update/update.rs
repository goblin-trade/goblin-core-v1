use crate::{
    goblin_error::GoblinError,
    quantities::{Exp, Quantity},
    settlement::ConstZero,
    types::{Marker, Tuple},
};

/// Update axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Update;

pub type Increase = Marker<Update, 0>;
pub type Decrease = Marker<Update, 1>;

#[derive(Clone, Copy)]
pub enum UpdateEnum {
    /// Increase store balance by decreasing resting order
    Increase,

    /// Decrease store balance by increasing resting order
    Decrease,
}

impl UpdateEnum {
    pub fn from_delta<E: Exp>(value: Quantity<E, i64>) -> Option<Self> {
        if value > Quantity::ZEROED {
            Some(Self::Increase)
        } else if value < Quantity::ZEROED {
            Some(Self::Decrease)
        } else {
            None
        }
    }
}

pub type UpdatePair<T0, T1> = Tuple<T0, T1, Update>;
pub type SameUpdatePair<T> = UpdatePair<T, T>;
