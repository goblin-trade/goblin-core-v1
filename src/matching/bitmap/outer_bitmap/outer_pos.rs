use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, Quote},
    matching::bitmap::Coordinate,
    quantities::Ticks,
    state::pair::OuterPosPair,
    types::StoreReader,
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct OuterPos {
    pub inner: u8,
}

impl OuterPos {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub fn byte_index(&self) -> usize {
        self.inner as usize / 8
    }

    pub fn bit_index(&self) -> usize {
        self.inner as usize % 8
    }

    pub fn holds_garbage(&self, pair: &OuterPosPair) -> bool {
        Base::closer_to_opposite_pole(*self, Base::get(pair))
            && Base::closer_to_opposite_pole(*self, Quote::get(pair))
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
