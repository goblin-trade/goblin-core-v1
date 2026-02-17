use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{column::Column, inner_pos::InnerPos, row::Row},
};

pub struct InnerCoordinates<In>
where
    In: LegMatcher,
{
    pub row: Row<In>,
    pub column: Column,
}

impl<In> From<InnerCoordinates<In>> for InnerPos<In>
where
    In: LegMatcher,
{
    fn from(value: InnerCoordinates<In>) -> Self {
        InnerPos::new(value.row.inner * 8 + value.column.inner)
    }
}
