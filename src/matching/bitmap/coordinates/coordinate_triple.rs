use crate::{
    axis::leg::leg_matcher::LegMatcher,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
        row_column::RowColumn, StoredCoordinates,
    },
    quantities::Ticks,
};

// impl<In> From<Ticks> for (OuterBitmapIndex<In>, OuterPos<In>, InnerPos<In>)
// where
//     In: LegMatcher,
// {
//     fn from(value: Ticks) -> Self {
//         (value.into(), value.into(), value.into())
//     }
// }

// impl<In> From<StoredCoordinates> for (OuterBitmapIndex<In>, OuterPos<In>, InnerPos<In>)
// where
//     In: LegMatcher,
// {
//     fn from(value: StoredCoordinates) -> Self {
//         (
//             value.price.into(),
//             value.price.into(),
//             RowColumn {
//                 row: value.price.into(),
//                 column: value.column,
//             }
//             .into(),
//         )
//     }
// }
