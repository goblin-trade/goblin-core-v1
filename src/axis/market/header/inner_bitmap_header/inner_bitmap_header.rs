use crate::{axis::leg::Pair, matching::bitmap::outer_pos::OuterPos};

pub struct InnerBitmapHeader {
    pub outer_pos: OuterPos,

    /// Number of update operations
    pub update_count: Pair<u8, u8>,
}
