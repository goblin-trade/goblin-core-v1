use crate::quantities::{InnerPos, POS_2, Pos1, PositionV2, SafePosition};

pub type Pos2 = SafePosition<POS_2>;

impl Pos2 {
    pub fn new(pos_1: Pos1, inner_pos: InnerPos) -> Self {
        Self {
            inner: PositionV2::from(pos_1) + inner_pos.into_position(),
        }
    }
}
