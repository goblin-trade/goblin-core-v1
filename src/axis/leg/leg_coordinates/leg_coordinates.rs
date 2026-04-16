use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::region::take_region::TakeRegion,
    quantities::{inner_val::InnerVal, DerivedPosition, TickPosV2},
};

pub trait LegCoordinates: LegQuantities {
    fn take_region(limit_price: TickPosV2, price: TickPosV2) -> TakeRegion;

    fn start<K, const BITS: u16>() -> DerivedPosition<K, BITS>
    where
        K: InnerVal;

    fn end<K, const BITS: u16>() -> DerivedPosition<K, BITS>
    where
        K: InnerVal;
}
