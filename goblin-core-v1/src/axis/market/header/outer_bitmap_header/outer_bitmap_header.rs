use crate::quantities::OuterBitmapIndex;

pub struct OuterBitmapHeader {
    pub outer_bitmap_index: OuterBitmapIndex,
    pub inner_bitmap_count: u8,
}
