use crate::{
    matching::bitmap::{column::Column, FullCoordinates},
    quantities::Ticks,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StoredCoordinates {
    pub price: Ticks,
    pub column: Column,
}

impl From<FullCoordinates> for StoredCoordinates {
    fn from(value: FullCoordinates) -> Self {
        Self {
            price: value.into(),
            column: value.inner_pos.into(),
        }
    }
}
