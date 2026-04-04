use crate::quantities::OuterBitmapIndexV2;

pub struct OuterBitmapHeader {
    pub outer_bitmap_index: OuterBitmapIndexV2,
    pub inner_bitmap_count: u8,
}
