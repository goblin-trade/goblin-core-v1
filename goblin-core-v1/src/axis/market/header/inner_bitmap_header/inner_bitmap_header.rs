use crate::quantities::OuterPos;

pub struct InnerBitmapHeader {
    pub outer_pos: OuterPos,

    /// Number of update operations
    pub update_count: u8,
}
