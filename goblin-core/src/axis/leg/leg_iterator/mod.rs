mod base;
mod quote;

use crate::{
    axis::leg::LegCoordinates,
    quantities::{
        BitsLayout, FullPos, FullPosition, INNER_POS, InnerPos, OUTER_BITMAP_INDEX, OUTER_POS,
        OuterBitmapIndex, OuterPos,
    },
};
use core::range::RangeInclusive;

/// Iterators of coordinates
///
/// Step trait is unstable. We are forced to declare dedicated types
/// and getter functions for each variant.
///
/// # Encoding of a range
///
/// Every method takes the same `range`, a lower/upper bound in full-position
/// units with `start <= last`, exactly as [`LegIterator::get_range`] produces.
/// Each method then walks the positions of one bit field that fall inside that
/// range, in the leg's direction (`Base` descends, `Quote` ascends).
pub trait LegIterator: LegCoordinates {
    /// Normalize last and limit positions into a RangeInclusive struct.
    ///
    /// In RangeInclusive, start is always the smaller value.
    fn get_range(last_position: FullPos, limit: FullPos) -> RangeInclusive<FullPos>;

    fn step_iter<const BITS: u16>(range: RangeInclusive<FullPos>) -> impl Iterator<Item = FullPos>;

    /// Walk the outer bitmap indices covered by `range`.
    fn outer_bitmap_index_iter(
        range: RangeInclusive<FullPos>,
    ) -> impl Iterator<Item = OuterBitmapIndex> {
        let offset = BitsLayout::<OUTER_BITMAP_INDEX>::OFFSET;
        let low = range.start.extract::<OUTER_BITMAP_INDEX>().inner << offset;
        let high = range.last.extract::<OUTER_BITMAP_INDEX>().inner << offset;

        Self::step_iter::<OUTER_BITMAP_INDEX>(RangeInclusive {
            start: FullPos::new(low),
            last: FullPos::new(high),
        })
        .map(|pos| pos.extract_and_convert::<u64, OUTER_BITMAP_INDEX>())
    }

    /// Walk the outer positions of the outer bitmap anchored at `current` that
    /// fall inside `range`.
    ///
    /// `current` is the base full position of the outer bitmap, i.e. its outer
    /// bitmap index shifted into place (`outer_bitmap_index << 16`).
    fn outer_pos_iter(
        range: RangeInclusive<FullPos>,
        current: FullPos,
    ) -> impl Iterator<Item = OuterPos> {
        let offset = BitsLayout::<OUTER_POS>::OFFSET;
        let head = current.inner;
        let tail = head + (BitsLayout::<OUTER_POS>::MASK << offset);

        Self::step_iter::<OUTER_POS>(RangeInclusive {
            start: FullPos::new(head),
            last: FullPos::new(tail),
        })
        .map(|pos| pos.extract_and_convert::<u8, OUTER_POS>())
        .filter(move |outer_pos| {
            // An outer position spans one full inner bitmap, so it is in range
            // if that whole span intersects the query range.
            let low = head + ((outer_pos.inner as u64) << offset);
            let high = low + BitsLayout::<OUTER_POS>::MASK;
            high >= range.start.inner && low <= range.last.inner
        })
    }

    /// Walk the inner positions of the inner bitmap anchored at `current` that
    /// fall inside `range`.
    ///
    /// `current` is the base full position of the inner bitmap, i.e. its outer
    /// bitmap index and outer position shifted into place with the inner bits
    /// zeroed.
    fn inner_pos_iter(
        range: RangeInclusive<FullPos>,
        current: FullPos,
    ) -> impl Iterator<Item = InnerPos> {
        let head = current.inner;
        let tail = head + BitsLayout::<INNER_POS>::MASK;

        Self::step_iter::<INNER_POS>(RangeInclusive {
            start: FullPos::new(head),
            last: FullPos::new(tail),
        })
        .map(|pos| pos.extract_and_convert::<u8, INNER_POS>())
        .filter(move |inner_pos| {
            let full = head + inner_pos.inner as u64;
            full >= range.start.inner && full <= range.last.inner
        })
    }
}
