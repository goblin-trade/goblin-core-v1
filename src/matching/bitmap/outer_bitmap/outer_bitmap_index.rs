use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, Pair, Quote},
    matching::bitmap::{Coordinate, StoredCoordinates},
    quantities::Ticks,
    types::StoreReader,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OuterBitmapIndex {
    pub inner: u64,
}

impl OuterBitmapIndex {
    pub const fn new(inner: u64) -> Self {
        Self { inner }
    }

    pub fn holds_garbage(&self, pair: &Pair<Self, Self>) -> bool {
        Base::closer_to_opposite_pole(*self, Base::get(pair))
            && Base::closer_to_opposite_pole(*self, Quote::get(pair))
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
