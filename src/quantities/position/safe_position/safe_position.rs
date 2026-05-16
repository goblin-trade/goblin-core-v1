use core::marker::PhantomData;

use crate::quantities::Position;

pub struct SafePosition<P> {
    pub(super) inner: Position,
    pub(super) _marker: PhantomData<P>,
}

impl<P> SafePosition<P> {
    pub fn position(&self) -> Position {
        self.inner
    }
}
