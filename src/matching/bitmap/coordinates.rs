use crate::{
    matching::bitmap::{InnerPos, OuterBitmapIndex, OuterPos},
    quantities::Ticks,
};

/// Coordinate representation of a price tick
pub struct Coordinates {
    pub outer_bitmap_index: OuterBitmapIndex,
    pub outer_pos: OuterPos,
    pub inner_pos: InnerPos,
}

impl From<Coordinates> for Ticks {
    fn from(value: Coordinates) -> Self {
        let outer_bitmap_index = value.outer_bitmap_index.0;
        let outer_pos = value.outer_pos.0 as u64;
        let inner_pos = value.inner_pos.0 as u64;

        let inner = outer_bitmap_index * (256 * 32) + outer_pos * 32 + inner_pos;

        Self::new(inner)
    }
}
