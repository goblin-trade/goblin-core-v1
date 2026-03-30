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

impl StoredCoordinates {
    pub fn closer_to_opposite_pole<In: LegMatcher>(&self, other: &Self) -> bool {
        let price = self.price;
        let other_price = other.price;

        if In::closer_to_opposite_pole(price, other_price) {
            return true;
        }

        if price == other_price && self.column < other.column {
            return true;
        }

        false
    }
}

impl From<FullCoordinates> for StoredCoordinates {
    fn from(value: FullCoordinates) -> Self {
        Self {
            price: value.into(),
            column: value.inner_pos.into(),
        }
    }
}
