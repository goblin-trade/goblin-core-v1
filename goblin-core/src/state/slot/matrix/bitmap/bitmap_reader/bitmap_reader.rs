use crate::{
    axis::leg::LegMatcher,
    axis_helpers::TokenPair,
    quantities::{Position, SafePosition},
    state::{MarketPreimage, SlotKey},
};
use core::range::RangeInclusive;

pub trait BitmapReader<const BITS: u16> {
    /// Give an iterator to return active positions inside a bitmap
    fn active_iterator<TP: TokenPair, In: LegMatcher>(
        market_key: SlotKey<MarketPreimage<TP>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = SafePosition<BITS>>;
}
