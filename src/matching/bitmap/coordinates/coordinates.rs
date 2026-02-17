use crate::matching::bitmap::{Column, PriceCoordinates};

pub struct Coordinates {
    pub price_coordinates: PriceCoordinates,
    pub column: Column,
}
