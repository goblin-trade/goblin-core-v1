use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base},
    matching::bitmap::{
        compact_coordinates::CompactCoordinates, outer_bitmap_index::OuterBitmapIndex,
        outer_pos::OuterPos, row::Row, Coordinate,
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

    fn coordinates_iter(
        item: CompactCoordinates<Self>,
    ) -> impl Iterator<Item = CompactCoordinates<Self>> {
        /// Mask to invert the MSB 5 bits that store row (0-31)
        /// The LSB 3 bits belong to column (0 - 7) and are unchanged.
        ///
        /// The value moves from 0 to 255, but we invert the row bits.
        /// This way column always goes from 0 to 7, but the direction of
        /// row is reversed from 31 to 0.
        ///
        /// We need to invert the starting position. For example if
        /// starting row is 30, we map it to 1 so we can iterate from 1 to 31,
        /// then perform the inversion.
        const MSB5_MASK: u8 = 0b1111_1000;
        let start_inverted = item.inner ^ MSB5_MASK;
        (start_inverted..=255).map(|inner| {
            let inner_inverted = inner ^ MSB5_MASK;
            CompactCoordinates::new(inner_inverted)
        })
    }

    fn closer_to_centre<K: PartialEq + PartialOrd>(first: K, second: K) -> bool {
        // For In=Base (ask), we match downwards against resting bids
        first > second
    }

    fn start_value<C: Coordinate>() -> C {
        C::MAX
    }
}
