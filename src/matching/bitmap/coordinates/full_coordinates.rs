use crate::{
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row,
        row_column::RowColumn, StoredCoordinates,
    },
    quantities::Ticks,
};

#[derive(Clone, Copy, PartialEq)]
pub struct FullCoordinates {
    pub outer_bitmap_index: OuterBitmapIndex,
    pub outer_pos: OuterPos,
    pub inner_pos: InnerPos,
}

impl From<Ticks> for FullCoordinates {
    fn from(value: Ticks) -> Self {
        Self {
            outer_bitmap_index: value.into(),
            outer_pos: value.into(),
            inner_pos: value.into(),
        }
    }
}

impl From<FullCoordinates> for Ticks {
    fn from(value: FullCoordinates) -> Self {
        let outer_bitmap_index = value.outer_bitmap_index.inner;
        let outer_pos = value.outer_pos.inner as u64;
        let row = Row::from(value.inner_pos).inner as u64;

        let inner = outer_bitmap_index * (256 * 32) + outer_pos * 32 + row;

        Self::new(inner)
    }
}

impl From<StoredCoordinates> for FullCoordinates {
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
