use crate::{
    axis::leg::Pair,
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, StoredCoordinates,
    },
};

pub type StoredCoordinatesPair = Pair<StoredCoordinates, StoredCoordinates>;
pub type OuterBitmapIndexPair = Pair<OuterBitmapIndex, OuterBitmapIndex>;
pub type OuterPosPair = Pair<OuterPos, OuterPos>;

impl From<StoredCoordinatesPair> for OuterBitmapIndexPair {
    fn from(value: StoredCoordinatesPair) -> Self {
        Pair::new(value.0.price.into(), value.1.price.into())
    }
}

impl From<StoredCoordinatesPair> for OuterPosPair {
    fn from(value: StoredCoordinatesPair) -> Self {
        Pair::new(value.0.price.into(), value.1.price.into())
    }
}
