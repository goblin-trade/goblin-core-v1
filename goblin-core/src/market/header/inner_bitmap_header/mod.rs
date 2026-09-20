mod process;

use deku::DekuRead;
#[cfg(feature = "encode")]
use deku::DekuWrite;

use crate::quantities::OuterPos;

#[derive(DekuRead)]
#[cfg_attr(feature = "encode", derive(DekuWrite))]
pub struct InnerBitmapHeader {
    pub outer_pos: OuterPos,

    /// Number of update operations
    pub update_count: u8,
}
