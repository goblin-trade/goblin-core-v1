mod alias;
pub use alias::*;

use crate::quantities::FullPos;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SafePosition<const BITS: u16> {
    pub(super) inner: FullPos,
}

impl<const BITS: u16> From<SafePosition<BITS>> for FullPos {
    fn from(value: SafePosition<BITS>) -> Self {
        value.inner
    }
}
