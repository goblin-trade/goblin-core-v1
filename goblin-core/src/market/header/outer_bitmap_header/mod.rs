mod process;

use crate::quantities::OuterBitmapIndexU32;
use goblin_macros::FixedDecode;

#[derive(FixedDecode)]
pub struct OuterBitmapHeader {
    pub outer_bitmap_index_u32: OuterBitmapIndexU32,
    pub inner_bitmap_count: u8,
}
