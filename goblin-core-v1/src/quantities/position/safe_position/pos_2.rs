use crate::quantities::{InnerPos, Pos1, Pos2, Position};

impl Pos2 {
    pub fn new(pos_1: Pos1, inner_pos: InnerPos) -> Self {
        Self {
            inner: Position::from(pos_1) + inner_pos.into(),
        }
    }
}
