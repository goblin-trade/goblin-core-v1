use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::region::take_region::TakeRegion,
    quantities::{bits_layout::BitsLayout, Position, TickPos},
};

impl LegCoordinates for Quote {
    fn take_region(limit_price: TickPos, price: TickPos) -> TakeRegion {
        if price < limit_price {
            TakeRegion::NotLeg
        } else {
            TakeRegion::Leg
        }
    }

    fn start<const BITS: u16>() -> Position {
        Position::ZERO
    }

    fn end<const BITS: u16>() -> Position {
        Position::new(BitsLayout::<BITS>::MAX)
    }
}
