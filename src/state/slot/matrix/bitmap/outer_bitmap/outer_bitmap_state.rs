use crate::state::bitmap::outer_bitmap::{active_outer_bitmap::ActiveOuterBitmap, OuterBitmap};

const EMPTY_VALUE: [u8; 32] = [0; 32];
const CLOSED_SENTINEL: [u8; 32] = [0xFF; 32];

pub enum OuterBitmapState {
    Empty,
    Closed,
    Active(ActiveOuterBitmap),
}

impl From<OuterBitmap> for OuterBitmapState {
    fn from(value: OuterBitmap) -> Self {
        match value.inner {
            EMPTY_VALUE => OuterBitmapState::Empty,
            CLOSED_SENTINEL => OuterBitmapState::Closed,
            _ => OuterBitmapState::Active(ActiveOuterBitmap::new(value.inner)),
        }
    }
}
