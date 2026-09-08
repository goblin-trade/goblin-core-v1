mod process;

use crate::quantities::OuterBitmapIndex;
use goblin_macros::FixedDecode;

#[derive(FixedDecode)]
pub struct OuterBitmapHeader {
    pub outer_bitmap_index: OuterBitmapIndex,
    pub inner_bitmap_count: u8,
}
