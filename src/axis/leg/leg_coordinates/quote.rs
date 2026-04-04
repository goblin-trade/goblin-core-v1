use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::region::take_region::TakeRegion,
    quantities::{inner_val::InnerVal, DerivedPosition, Ticks},
};

impl LegCoordinates for Quote {
    fn take_region(limit_price: Ticks, price: Ticks) -> TakeRegion {
        if price < limit_price {
            TakeRegion::NotLeg
        } else {
            TakeRegion::Leg
        }
    }

    fn start<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>(
    ) -> DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
    where
        K: InnerVal,
    {
        DerivedPosition::min()
    }

    fn end<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>(
    ) -> DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
    where
        K: InnerVal,
    {
        DerivedPosition::max()
    }
}
