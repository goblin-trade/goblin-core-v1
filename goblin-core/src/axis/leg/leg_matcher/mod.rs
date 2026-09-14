use crate::{
    axis::leg::{
        Base, LegConstants, LegCoordinates, LegEnum, LegIterator, LegMath, LegQuantities,
        LegReader, LegValidator, Quote,
    },
    axis_helpers::AxisMarker,
    quantities::Exp,
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
    + Exp
{
}

impl Exp for Base {}
impl Exp for Quote {}

impl LegMatcher for Base {}
impl LegMatcher for Quote {}
