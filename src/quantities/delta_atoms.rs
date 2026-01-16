use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct DeltaAtoms {
    inner: i64,
}

impl DeltaAtoms {
    pub const ZERO: Self = DeltaAtoms { inner: 0 };

    pub fn new(inner: i64) -> Self {
        Self { inner }
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.inner.checked_add(rhs.inner).map(DeltaAtoms::new)
    }
}

impl<'a> Decodable<'a> for DeltaAtoms {
    fn try_decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        i64::try_decode(ctx).map(DeltaAtoms::new)
    }
}
