use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{column::Column, compact_coordinates::CompactCoordinates, row::Row},
};

#[derive(Clone, Copy)]
pub struct InnerCoordinates<In>
where
    In: LegMatcher,
{
    pub row: Row<In>,
    pub column: Column,
}

impl<In> From<CompactCoordinates<In>> for InnerCoordinates<In>
where
    In: LegMatcher,
{
    fn from(value: CompactCoordinates<In>) -> Self {
        Self {
            row: Row::from(value),
            column: Column::from(value),
        }
    }
}

impl<In> InnerCoordinates<In>
where
    In: LegMatcher,
{
    pub fn start_value() -> Self {
        Self {
            row: In::start_value(),
            column: Column::new(0),
        }
    }
}
