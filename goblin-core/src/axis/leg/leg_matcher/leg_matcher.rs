use crate::{
    axis::leg::{
        Base, LegConstants, LegCoordinates, LegEnum, LegIterator, LegMath, LegQuantities,
        LegReader, LegValidator, Quote,
    },
    axis_helpers::AxisMarker,
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
