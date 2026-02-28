use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{column::Column, FullCoordinates},
    quantities::Ticks,
};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct StoredCoordinates {
    pub price: Ticks,
    pub column: Column,
}

impl<In> From<FullCoordinates<In>> for StoredCoordinates
where
    In: LegMatcher,
{
    fn from(value: FullCoordinates<In>) -> Self {
        Self {
            price: value.into(),
            column: value.inner_pos.into(),
        }
    }
}
