use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::region::take_region::TakeRegion,
    quantities::{inner_val::InnerVal, DerivedPosition, Position, TickPosV2},
};

impl LegCoordinates for Quote {
    fn take_region(limit_price: TickPosV2, price: TickPosV2) -> TakeRegion {
        if price < limit_price {
            TakeRegion::NotLeg
        } else {
            TakeRegion::Leg
        }
    }

    fn start<K, const BITS: u16>() -> DerivedPosition<K, BITS>
    where
        K: InnerVal,
    {
        DerivedPosition::min()
    }

    fn end<K, const BITS: u16>() -> DerivedPosition<K, BITS>
    where
        K: InnerVal,
    {
        DerivedPosition::max()
    }

    fn start_v2<const BITS: u16>() -> Position {
        Position::ZERO
    }

    fn end_v2<const BITS: u16>() -> Position {
        Position::new(DerivedPosition::<u64, BITS>::MAX_RAW)
    }
}
