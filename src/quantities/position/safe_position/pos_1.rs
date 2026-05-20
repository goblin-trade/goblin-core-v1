use crate::quantities::{OuterPos, Position, SafePosition, POS_0, POS_1};

impl SafePosition<POS_1> {
    pub fn new(pos_0: SafePosition<POS_0>, outer_pos: OuterPos) -> Self {
        Self {
            inner: Position::from(pos_0) + outer_pos.into(),
        }
    }
}
