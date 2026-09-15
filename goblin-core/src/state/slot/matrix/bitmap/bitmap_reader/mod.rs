mod impl_inner_pos;
mod impl_outer_pos;

use crate::{
    axis::leg::LegMatcher,
    axis_helpers::TokenPair,
    quantities::{PositionV2, SafePosition},
    state::{MarketPreimage, SlotKey},
};
use core::range::RangeInclusive;

pub trait BitmapReader<const BITS: u16> {
    /// Give an iterator to return active positions inside a bitmap
    fn active_iterator<TP: TokenPair, In: LegMatcher>(
        market_key: SlotKey<MarketPreimage<TP>>,
        range: RangeInclusive<PositionV2>,
    ) -> impl Iterator<Item = SafePosition<BITS>>;
}
