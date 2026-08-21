use crate::quantities::{OuterBitmapIndex, Pos0};

impl Pos0 {
    pub fn new(outer_bitmap_index: OuterBitmapIndex) -> Self {
        Self {
            inner: outer_bitmap_index.into(),
        }
    }
}
