use crate::{
    axis::leg::{leg_coordinates::LegCoordinates, Base, Quote, SamePair},
    types::StoreReader,
};

// TODO apply on
// 1. OuterBitmapIndex
// 2. (OuterBitmapIndex, OuterPos)
// 3. ((OuterBitmapIndex, OuterPos), InnerPos)
pub trait BitmapPos: Clone + Copy + Sized + PartialEq + PartialOrd {
    fn is_garbage(&self, limits: &SamePair<Self>) -> bool {
        Base::closer_to_opposite_pole(*self, Base::get(limits))
            && Quote::closer_to_opposite_pole(*self, Quote::get(limits))
    }
}
