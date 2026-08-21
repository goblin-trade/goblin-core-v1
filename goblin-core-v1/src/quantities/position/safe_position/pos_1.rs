use crate::quantities::{OuterPos, Pos0, Pos1, Position};

impl Pos1 {
    pub fn new(pos_0: Pos0, outer_pos: OuterPos) -> Self {
        Self {
            inner: Position::from(pos_0) + outer_pos.into(),
        }
    }
}
