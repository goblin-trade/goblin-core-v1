use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2},
    state::{bitmap_v2::outer_index::OuterIndex, MarketPreimage, SlotKey},
};
use core::default;
use core::ops::RangeInclusive;

pub trait OrderedIndex: Clone + Copy + PartialEq + Default {
    type Prev: OrderedIndex<Prev: Clone + Copy + PartialEq>;

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher;

    fn parent_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(OuterIndex<Self>, Self)>,
    ) -> impl Iterator<Item = (OuterIndex<Self>, impl Iterator<Item = Self>)>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}

impl OrderedIndex for () {
    type Prev = (); // bottoms out, self-referential terminator

    fn linear_iterator<In>(_range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        core::iter::once(()) // or empty(), depending on your semantics
    }

    fn parent_iterator<M, B, Q, In>(
        _market_key: SlotKey<MarketPreimage<M, B, Q>>,
        _range: RangeInclusive<(OuterIndex<Self>, Self)>,
    ) -> impl Iterator<Item = (OuterIndex<Self>, impl Iterator<Item = Self>)>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        core::iter::once((OuterIndex::<Self>::default(), core::iter::once(())))
    }
}

impl OrderedIndex for OuterBitmapIndexV2 {
    type Prev = ();

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_bitmap_index_iter(range)
    }

    fn parent_iterator<M, B, Q, In>(
        _market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(OuterIndex<Self>, Self)>,
    ) -> impl Iterator<Item = (OuterIndex<Self>, impl Iterator<Item = Self>)>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let outer_index = OuterIndex::<Self>::default();

        let (start, end) = range.clone().into_inner();
        let outer_bitmap_index_iterator = Self::linear_iterator::<In>(start.1..=end.1);

        core::iter::once((outer_index, outer_bitmap_index_iterator))
    }
}

impl OrderedIndex for OuterPosV2 {
    type Prev = OuterBitmapIndexV2;

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::outer_pos_iter(range)
    }
}

impl OrderedIndex for InnerPosV2 {
    type Prev = OuterPosV2;

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        In::inner_pos_iter(range)
    }
}
