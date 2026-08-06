use crate::quantities::OuterPos;
use goblin_macros::FixedDecode;

#[derive(FixedDecode)]
pub struct InnerBitmapHeader {
    pub outer_pos: OuterPos,

    /// Number of update operations
    pub update_count: u8,
}
