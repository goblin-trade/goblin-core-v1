use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Pos2, Position},
    state::{MarketPreimage, SlotKey},
};
use core::ops::RangeInclusive;

pub trait BitmapReader {
    /// Give an iterator to return active positions inside a bitmap
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Pos2>,
    ) -> impl Iterator<Item = Pos2>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}
