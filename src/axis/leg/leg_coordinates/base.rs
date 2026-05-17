use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    quantities::{bits_layout::BitsLayout, Position, SafePosition, POS_2},
};

impl LegCoordinates for Base {
    // fn take_region(limit_price: TickPos, price: TickPos) -> TakeRegion {
    //     if price > limit_price {
    //         TakeRegion::NotLeg
    //     } else {
    //         TakeRegion::Leg
    //     }
    // }

    fn in_region(last_position: SafePosition<POS_2>, position: SafePosition<POS_2>) -> bool {
        position <= last_position
    }

    fn start<const BITS: u16>() -> Position {
        Position::new(BitsLayout::<BITS>::MAX)
    }

    fn end<const BITS: u16>() -> Position {
        Position::ZERO
    }
}
