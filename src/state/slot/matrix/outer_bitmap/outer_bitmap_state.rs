use crate::state::outer_bitmap::{active_outer_bitmap::ActiveOuterBitmap, OuterBitmap};

pub const CLOSED_SENTINEL: OuterBitmap = OuterBitmap([0xFF; 32]);

pub enum OuterBitmapState {
    Closed,
    Active(ActiveOuterBitmap),
}

impl From<OuterBitmap> for OuterBitmapState {
    fn from(value: OuterBitmap) -> Self {
        if value == CLOSED_SENTINEL {
            OuterBitmapState::Closed
        } else {
            OuterBitmapState::Active(ActiveOuterBitmap(value.0))
        }
    }
}
