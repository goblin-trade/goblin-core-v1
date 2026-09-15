mod base;
mod quote;

use crate::{
    axis::leg::LegCoordinates,
    quantities::{
        FullPos, FullPosition, INNER_POS, InnerPos, OUTER_BITMAP_INDEX, OUTER_POS,
        OuterBitmapIndex, OuterPos, PositionRange,
    },
};
use core::range::RangeInclusive;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
pub trait LegIterator: LegCoordinates {
    /// Normalize last and limit positions into a RangeInclusive struct.
    ///
    /// In RangeInclusive, start is always the smaller value.
    fn get_range(last_position: FullPos, limit: FullPos) -> RangeInclusive<FullPos>;

    fn step_iter<const BITS: u16>(range: RangeInclusive<FullPos>) -> impl Iterator<Item = FullPos>;

    fn outer_bitmap_index_iter(
        range: RangeInclusive<FullPos>,
    ) -> impl Iterator<Item = OuterBitmapIndex> {
        let extracted_range = range.extract_range::<OUTER_BITMAP_INDEX>();
        Self::step_iter::<OUTER_BITMAP_INDEX>(extracted_range).map(|pos| pos.extract_and_convert())
    }

    fn outer_pos_iter(
        range: RangeInclusive<FullPos>,
        current: FullPos,
    ) -> impl Iterator<Item = OuterPos> {
        let clamped_range = range.clamp_range::<Self, OUTER_POS>(current);
        Self::step_iter::<OUTER_POS>(clamped_range).map(|pos| pos.extract_and_convert())
    }

    fn inner_pos_iter(
        range: RangeInclusive<FullPos>,
        current: FullPos,
    ) -> impl Iterator<Item = InnerPos> {
        let clamped_range = range.clamp_range::<Self, INNER_POS>(current);
        Self::step_iter::<INNER_POS>(clamped_range).map(|pos| pos.extract_and_convert())
    }
}
