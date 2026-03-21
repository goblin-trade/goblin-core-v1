use crate::{matching::bitmap::Coordinate, quantities::Ticks};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OuterBitmapIndex {
    pub inner: u64,
}

impl OuterBitmapIndex {
    pub const fn new(inner: u64) -> Self {
        Self { inner }
    }
}

impl Coordinate for OuterBitmapIndex {
    type Inner = u64;
    const MIN: Self = OuterBitmapIndex::new(0);
    const MAX: Self = OuterBitmapIndex::new(u64::MAX);

    fn inner(self) -> Self::Inner {
        self.inner
    }
}

impl From<Ticks> for OuterBitmapIndex {
    fn from(value: Ticks) -> Self {
        Self::new(value.inner / (256 * 32))
    }
}
