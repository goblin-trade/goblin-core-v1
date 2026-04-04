use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::region::take_region::TakeRegion,
    quantities::{inner_val::InnerVal, DerivedPosition, Ticks},
};

pub trait LegCoordinates: LegQuantities {
    fn take_region(limit_price: Ticks, price: Ticks) -> TakeRegion;

    fn start<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>(
    ) -> DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
    where
        K: InnerVal;

    fn end<K, const BIT_OFFSET: usize, const BIT_COUNT: usize>(
    ) -> DerivedPosition<K, BIT_OFFSET, BIT_COUNT>
    where
        K: InnerVal;
}
