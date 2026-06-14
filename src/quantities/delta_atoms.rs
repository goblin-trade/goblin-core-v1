use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    settlement::{CheckedAdd, ConstZero},
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct DeltaAtoms {
    inner: i64,
}

impl ConstZero for DeltaAtoms {
    const ZEROED: Self = DeltaAtoms { inner: 0 };
}

impl DeltaAtoms {
    pub fn new(inner: i64) -> Self {
        Self { inner }
    }
}

impl Decodable for DeltaAtoms {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        i64::try_decode(ctx).map(DeltaAtoms::new)
    }
}

impl CheckedAdd for DeltaAtoms {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(DeltaAtoms::new)
    }
}
