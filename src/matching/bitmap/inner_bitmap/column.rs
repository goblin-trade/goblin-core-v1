use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{inner_pos::InnerPos, Coordinate},
};

pub struct Column {
    pub inner: u8,
}

impl Column {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }
}

impl Coordinate for Column {
    const MIN: Self = Self::new(0);

    const MAX: Self = Self::new(7);

    fn iter(self) -> impl Iterator<Item = Self> {
        (Self::MIN.inner..=Self::MAX.inner).map(Self::new)
    }
}

impl<In> From<InnerPos<In>> for Column
where
    In: LegMatcher,
{
    fn from(value: InnerPos<In>) -> Self {
        Column::new(value.inner % 8)
    }
}
