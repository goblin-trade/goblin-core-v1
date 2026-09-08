use crate::quantities::{OuterBitmapIndex, POS_0, SafePosition};

pub type Pos0 = SafePosition<POS_0>;

impl Pos0 {
    pub fn new(outer_bitmap_index: OuterBitmapIndex) -> Self {
        Self {
            inner: outer_bitmap_index.into(),
        }
    }
}
