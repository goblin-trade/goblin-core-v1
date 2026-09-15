use crate::quantities::{OuterPos, POS_1, Pos0, PositionV2, SafePosition};

pub type Pos1 = SafePosition<POS_1>;

impl Pos1 {
    pub fn new(pos_0: Pos0, outer_pos: OuterPos) -> Self {
        Self {
            inner: PositionV2::from(pos_0) + outer_pos.into_position(),
        }
    }
}
