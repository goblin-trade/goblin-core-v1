use crate::quantities::{OuterPos, SafePosition, POS_0, POS_1};

impl SafePosition<POS_1> {
    pub fn new(pos_0: SafePosition<POS_0>, outer_pos: OuterPos) -> Self {
        Self {
            inner: pos_0.position() + outer_pos.into(),
        }
    }
}
