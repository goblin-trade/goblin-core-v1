use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{column::Column, row::Row},
};

#[derive(Clone, Copy)]
pub struct InnerCoordinates<In>
where
    In: LegMatcher,
{
    pub row: Row<In>,
    pub column: Column,
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
