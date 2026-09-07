use crate::quantities::Position;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct SafePosition<const BITS: u16> {
    pub(super) inner: Position,
}

impl<const BITS: u16> From<SafePosition<BITS>> for Position {
    fn from(value: SafePosition<BITS>) -> Self {
        value.inner
    }
}
