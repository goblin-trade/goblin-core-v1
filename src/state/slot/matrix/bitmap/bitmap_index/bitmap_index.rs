use core::ops::RangeInclusive;

use crate::axis::leg::leg_iterator::LegIterator;

pub trait BitmapIndex: Clone + Copy + PartialEq + PartialOrd {
    fn build_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegIterator;
}
