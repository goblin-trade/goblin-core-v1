mod process;

use crate::quantities::OuterPos;
use goblin_macros::fixed_codec;

#[fixed_codec]
pub struct InnerBitmapHeader {
    pub outer_pos: OuterPos,

    /// Number of update operations
    pub update_count: u8,
}
