use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::bitmap::{
        column::Column, compact_coordinates::CompactCoordinates,
        inner_coordinates::InnerCoordinates, outer_bitmap_index::OuterBitmapIndex,
        outer_pos::OuterPos, row::Row, Coordinate,
    },
};

impl LegCoordinates for Quote {
    fn outer_bitmap_index_iter(
        item: OuterBitmapIndex<Self>,
    ) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (item.inner..=OuterBitmapIndex::<Self>::MAX.inner).map(OuterBitmapIndex::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>> {
        (item.inner..=OuterPos::<Self>::MAX.inner).map(OuterPos::new)
    }

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>> {
        (item.inner..=Row::<Self>::MAX.inner).map(Row::new)
    }

    fn coordinates_iter(
        item: CompactCoordinates<Self>,
    ) -> impl Iterator<Item = CompactCoordinates<Self>> {
        (item.inner..=255).map(CompactCoordinates::new)
    }

    // fn coordinates_iter(
    //     item: InnerCoordinates<Self>,
    // ) -> impl Iterator<Item = InnerCoordinates<Self>> {
    //     let start_row = item.row;

    //     item.row.iter().flat_map(move |row| {
    //         let column_start = if row.inner == start_row.inner {
    //             item.column
    //         } else {
    //             Column::MIN
    //         };

    //         column_start
    //             .iter()
    //             .map(move |column| InnerCoordinates { row, column })
    //     })
    // }

    fn closer_to_centre<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Quote (bid), we match upwards against resting asks
        first < second
    }

    fn start_value<C: Coordinate>() -> C {
        C::MIN
    }
}
