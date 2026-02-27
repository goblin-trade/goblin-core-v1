use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        row_column::RowColumn, StoredCoordinates,
    },
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq)]
pub struct FullCoordinates<In>
where
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_pos: OuterPos<In>,
    pub inner_pos: InnerPos<In>,
}

impl<In> From<Ticks> for FullCoordinates<In>
where
    In: LegMatcher,
{
    fn from(value: Ticks) -> Self {
        Self {
            outer_bitmap_index: value.into(),
            outer_pos: value.into(),
            inner_pos: value.into(),
        }
    }
}

impl<In> From<StoredCoordinates> for FullCoordinates<In>
where
    In: LegMatcher,
{
    fn from(value: StoredCoordinates) -> Self {
        Self {
            outer_bitmap_index: value.price.into(),
            outer_pos: value.price.into(),
            inner_pos: RowColumn {
                row: value.price.into(),
                column: value.column,
            }
            .into(),
        }
    }
}
