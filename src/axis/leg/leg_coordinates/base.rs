use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::region::take_region::TakeRegion,
    quantities::{bits_layout::BitsLayout, Position, TickPosV2},
};

impl LegCoordinates for Base {
    fn take_region(limit_price: TickPosV2, price: TickPosV2) -> TakeRegion {
        if price > limit_price {
            TakeRegion::NotLeg
        } else {
            TakeRegion::Leg
        }
    }

    fn start<const BITS: u16>() -> Position {
        Position::new(BitsLayout::<BITS>::MAX)
    }

    fn end<const BITS: u16>() -> Position {
        Position::ZERO
    }
}
