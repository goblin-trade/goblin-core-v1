use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{column::Column, inner_pos::InnerPos, row::Row},
};

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct RowColumn<In>
where
    In: LegMatcher,
{
    pub row: Row<In>,
    pub column: Column,
}

impl<In> From<InnerPos<In>> for RowColumn<In>
where
    In: LegMatcher,
{
    fn from(value: InnerPos<In>) -> Self {
        Self {
            row: Row::from(value),
            column: Column::from(value),
        }
    }
}

impl<In> RowColumn<In>
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
