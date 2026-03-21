use crate::{matching::bitmap::Coordinate, quantities::Ticks};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OuterPos {
    pub inner: u8,
}

impl OuterPos {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }
}

impl Coordinate for OuterPos {
    type Inner = u8;
    const MIN: Self = OuterPos::new(0);
    const MAX: Self = OuterPos::new(255);

    fn inner(self) -> Self::Inner {
        self.inner
    }
}

impl From<Ticks> for OuterPos {
    fn from(value: Ticks) -> Self {
        Self::new(((value.inner / 32) % 256) as u8)
    }
}
