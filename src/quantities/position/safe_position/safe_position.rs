use crate::quantities::Position;

#[derive(Clone, Copy)]
pub struct SafePosition<const BITS: u16> {
    pub(super) inner: Position,
}

impl<const BITS: u16> SafePosition<BITS> {
    pub fn position(&self) -> Position {
        self.inner
    }
}
