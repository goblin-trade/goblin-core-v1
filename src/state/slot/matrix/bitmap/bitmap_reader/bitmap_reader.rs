use crate::{
    axis::{leg::leg_matcher::LegMatcher, market::market_spec::MarketSpec},
    quantities::{Position, SafePosition},
    state::{MarketPreimage, SlotKey},
};
use core::ops::RangeInclusive;

pub trait BitmapReader<const BITS: u16> {
    /// Give an iterator to return active positions inside a bitmap
    fn active_iterator<MS: MarketSpec, In: LegMatcher>(
        market_key: SlotKey<MarketPreimage<MS>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = SafePosition<BITS>>;
}
