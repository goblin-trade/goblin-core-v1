mod process;

use crate::quantities::OuterBitmapIndexU32;
use goblin_macros::fixed_codec;

#[fixed_codec]
pub struct OuterBitmapHeader {
    pub outer_bitmap_index_u32: OuterBitmapIndexU32,
    pub inner_bitmap_count: u8,
}
