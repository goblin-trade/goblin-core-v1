mod process;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::quantities::OuterBitmapIndexU32;

#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct OuterBitmapHeader {
    pub outer_bitmap_index_u32: OuterBitmapIndexU32,
    pub inner_bitmap_count: u8,
}
