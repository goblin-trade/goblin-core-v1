use crate::quantities::OuterPosV2;

pub struct InnerBitmapHeader {
    pub outer_pos: OuterPosV2,

    /// Number of update operations
    pub update_count: u8,
}
