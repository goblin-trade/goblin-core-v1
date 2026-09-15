mod alias;
pub use alias::*;

use crate::quantities::PositionV2;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SafePosition<const BITS: u16> {
    pub(super) inner: PositionV2,
}

impl<const BITS: u16> From<SafePosition<BITS>> for PositionV2 {
    fn from(value: SafePosition<BITS>) -> Self {
        value.inner
    }
}
