use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Quote},
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row, Coordinate,
    },
};

impl LegCoordinates for Quote {
    fn outer_bitmap_index_iter(
        item: OuterBitmapIndex<Self>,
    ) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (item.inner..=OuterBitmapIndex::<Self>::MAX.inner).map(OuterBitmapIndex::<Self>::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>> {
        (item.inner..=OuterPos::<Self>::MAX.inner).map(OuterPos::<Self>::new)
    }

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>> {
        (item.inner..=Row::<Self>::MAX.inner).map(Row::<Self>::new)
    }

    fn closer_to_centre_v2<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Quote (bid), we match upwards against resting asks
        first < second
    }
}
