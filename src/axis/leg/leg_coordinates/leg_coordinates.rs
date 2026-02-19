use crate::{
    axis::leg::leg_quantities::LegQuantities,
    matching::bitmap::{outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, row::Row},
};

pub trait LegCoordinates: LegQuantities {
    fn outer_bitmap_index_iter(
        item: OuterBitmapIndex<Self>,
    ) -> impl Iterator<Item = OuterBitmapIndex<Self>>;

    fn outer_pos_iter(item: OuterPos<Self>) -> impl Iterator<Item = OuterPos<Self>>;

    fn row_iter(item: Row<Self>) -> impl Iterator<Item = Row<Self>>;

    /// Whether `first` is closer to the centre than `second`
    ///
    /// # Convention
    ///
    /// `In` is the taker direction
    ///
    /// 1. For In = Base (ask / sell) match against resting bids downwards. first > second.
    /// 2. For In = Quote (bid / buy) match against resting asks upwards. first < second.
    fn closer_to_centre<K: PartialEq + PartialOrd>(first: K, second: K) -> bool;
}
