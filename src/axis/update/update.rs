use crate::types::Marker;

/// Update axis
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Update;

pub type Increase = Marker<Update, 0>;
pub type Decrease = Marker<Update, 1>;

pub enum UpdateEnum {
    Increase(Increase),
    Decrease(Decrease),
}
