use crate::axis::{
    leg::{
        leg_constants::LegConstants, leg_coordinates::LegCoordinates, leg_iterator::LegIterator,
        leg_math::LegMath, leg_quantities::LegQuantities, leg_reader::LegReader,
        leg_validator::LegValidator, Base, LegEnum, Quote,
    },
    AxisMarker,
};

/// Supertrait for leg operations
pub trait LegMatcher:
    AxisMarker<Enum = LegEnum>
    + LegQuantities
    + LegMath
    + LegConstants
    + LegValidator
    + LegCoordinates
    + LegIterator
    + LegReader
{
}

impl LegMatcher for Base {}
impl LegMatcher for Quote {}
