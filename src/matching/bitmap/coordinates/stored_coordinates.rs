use crate::{matching::bitmap::column::Column, quantities::Ticks};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StoredCoordinates {
    pub price: Ticks,
    pub column: Column,
}
