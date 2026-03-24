use crate::types::Marker;

/// Update axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Update;

pub type Increase = Marker<Update, 0>;
pub type Decrease = Marker<Update, 1>;

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
