use crate::quantities::{FullPos, InnerPos, POS_2, Pos1, SafePosition};

pub type Pos2 = SafePosition<POS_2>;

impl Pos2 {
    pub fn new(pos_1: Pos1, inner_pos: InnerPos) -> Self {
        Self {
            inner: FullPos::from(pos_1) + inner_pos.into_position(),
        }
    }
}
