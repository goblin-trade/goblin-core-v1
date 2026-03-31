use crate::{
    axis::leg::{Pair, SamePair},
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, StoredCoordinates,
    },
};

impl From<SamePair<StoredCoordinates>> for SamePair<OuterBitmapIndex> {
    fn from(value: SamePair<StoredCoordinates>) -> Self {
        Pair::new(value.0.price.into(), value.1.price.into())
    }
}

impl From<SamePair<StoredCoordinates>> for SamePair<OuterPos> {
    fn from(value: SamePair<StoredCoordinates>) -> Self {
        Pair::new(value.0.price.into(), value.1.price.into())
    }
}
