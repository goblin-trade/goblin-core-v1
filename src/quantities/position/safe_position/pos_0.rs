use crate::quantities::{OuterBitmapIndex, SafePosition, POS_0};

impl SafePosition<POS_0> {
    pub fn new(outer_bitmap_index: OuterBitmapIndex) -> Self {
        Self {
            inner: outer_bitmap_index.into(),
        }
    }
}
