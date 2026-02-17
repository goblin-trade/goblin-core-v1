use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{column::Column, PriceCoordinates},
};

pub struct Coordinates<In>
where
    In: LegMatcher,
{
    pub price_coordinates: PriceCoordinates<In>,
    pub column: Column,
}
