use crate::types::{Marker, Tuple};

/// Update axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Update;

pub type Increase = Marker<Update, 0>;
pub type Decrease = Marker<Update, 1>;

#[derive(Clone, Copy)]
pub enum UpdateEnum {
    Increase,
    Decrease,
}

impl From<bool> for UpdateEnum {
    fn from(value: bool) -> Self {
        if value {
            UpdateEnum::Increase
        } else {
            UpdateEnum::Decrease
        }
    }
}

pub type UpdatePair<T0, T1> = Tuple<T0, T1, Update>;
pub type SameUpdatePair<T> = UpdatePair<T, T>;
