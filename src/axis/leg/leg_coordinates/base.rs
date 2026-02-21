use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row, Coordinate,
    },
};

impl LegCoordinates for Base {
    fn outer_bitmap_index_iter(
        item: OuterBitmapIndex<Self>,
    ) -> impl Iterator<Item = OuterBitmapIndex<Self>> {
        (OuterBitmapIndex::<Self>::MIN.inner..=item.inner)
            .rev()
            .map(OuterBitmapIndex::<Self>::new)
    }

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>> {
        (OuterPos::<Self>::MIN.inner..=item.inner)
            .rev()
            .map(OuterPos::<Self>::new)
    }

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>> {
        (Row::<Self>::MIN.inner..=item.inner)
            .rev()
            .map(Row::<Self>::new)
    }

    fn closer_to_centre<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Base (ask), we match downwards against resting bids
        first > second
    }

    fn start_value<C: Coordinate>() -> C {
        C::MAX
    }
}
