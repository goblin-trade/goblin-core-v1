use crate::{
    matching::bitmap::{OuterBitmapIndex, OuterPos, Row},
    quantities::Ticks,
};

/// Coordinate representation of a price tick
pub struct PriceCoordinates {
    pub outer_bitmap_index: OuterBitmapIndex,
    pub outer_pos: OuterPos,
    pub row: Row,
}

impl From<Ticks> for PriceCoordinates {
    fn from(value: Ticks) -> Self {
        Self {
            outer_bitmap_index: value.into(),
            outer_pos: value.into(),
            row: value.into(),
        }
    }
}

impl From<PriceCoordinates> for Ticks {
    fn from(value: PriceCoordinates) -> Self {
        let outer_bitmap_index = value.outer_bitmap_index.0;
        let outer_pos = value.outer_pos.0 as u64;
        let row = value.row.0 as u64;

        let inner = outer_bitmap_index * (256 * 32) + outer_pos * 32 + row;

        Self::new(inner)
    }
}
