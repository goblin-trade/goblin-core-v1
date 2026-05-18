use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Position, SafePosition},
    state::{MarketPreimage, SlotKey},
};
use core::ops::RangeInclusive;

pub trait BitmapReader<const BITS: u16> {
    /// Give an iterator to return active positions inside a bitmap
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = SafePosition<BITS>>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}
