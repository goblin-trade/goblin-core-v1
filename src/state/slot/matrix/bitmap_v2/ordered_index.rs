use core::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2},
    state::MarketPreimage,
};

// Add inner() function
// But we need to duplicate code for each impl
pub trait OrderedIndex: Clone + Copy + PartialEq {
    type Prev: Clone + Copy + PartialEq;
    type PrevKey<M, B, Q>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher;
}

pub type OuterIndex<I: OrderedIndex> = (<I::Prev as OrderedIndex>::Prev, I::Prev);

// preimage
// 1. OuterBitmapIndex: SlotKey<MarketPreimage> +

// Do we need a new trait to map Preimage?
// The current key is a function of the previous key.
// However for OrderedIndex the key is seeded as MarketPreimage
//
// If prev is just Clone + Copy, use seed value
// If Prev is also of type OrderedIndex, use its key

impl OrderedIndex for OuterBitmapIndexV2 {
    type Prev = ();
    type PrevKey<M, B, Q>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
    = MarketPreimage<M, B, Q>;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_bitmap_index_iter(range)
    }
}

impl OrderedIndex for OuterPosV2 {
    type Prev = OuterBitmapIndexV2;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_pos_iter(range)
    }
}

impl OrderedIndex for InnerPosV2 {
    type Prev = OuterPosV2;

    fn get_iter<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::inner_pos_iter(range)
    }
}
