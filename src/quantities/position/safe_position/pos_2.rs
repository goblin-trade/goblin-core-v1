use crate::quantities::{InnerPos, SafePosition, POS_1, POS_2};

impl SafePosition<POS_2> {
    pub fn new(pos_1: SafePosition<POS_1>, inner_pos: InnerPos) -> Self {
        Self {
            inner: pos_1.position() + inner_pos.into(),
        }
    }
}
