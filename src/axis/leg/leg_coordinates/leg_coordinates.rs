use crate::{
    axis::leg::{leg_quantities::LegQuantities, SamePair},
    quantities::{Position, SafePosition, TickPos, POS_2},
};

pub trait LegCoordinates: LegQuantities {
    // fn take_region(limit_price: TickPos, price: TickPos) -> TakeRegion;

    fn in_region(last_position: SafePosition<POS_2>, position: SafePosition<POS_2>) -> bool;

    fn start<const BITS: u16>() -> Position;

    fn end<const BITS: u16>() -> Position;
}
