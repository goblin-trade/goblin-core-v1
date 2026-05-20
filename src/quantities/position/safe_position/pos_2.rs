use crate::quantities::{InnerPos, Position, SafePosition, POS_1, POS_2};

impl SafePosition<POS_2> {
    pub fn new(pos_1: SafePosition<POS_1>, inner_pos: InnerPos) -> Self {
        Self {
            inner: Position::from(pos_1) + inner_pos.into(),
        }
    }
}
