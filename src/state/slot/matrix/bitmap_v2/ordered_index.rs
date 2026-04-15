use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{InnerPosV2, OuterBitmapIndexV2, OuterPosV2, INNER_POS_V2, OUTER_POS_V2},
    state::{
        bitmap_v2::{outer_index::OuterIndex, preimage::BitmapPreimageV2},
        MarketPreimage, Preimage, SlotKey,
    },
};
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

    fn parent_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(OuterIndex<Self>, Self)>,
    ) -> impl Iterator<Item = (OuterIndex<Self>, impl Iterator<Item = Self>)>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let (start, end) = range.clone().into_inner();
        let outer_range = start.0 .1..=end.0 .1;

        OuterBitmapIndexV2::linear_iterator::<In>(outer_range).filter_map(
            move |outer_bitmap_index| {
                let outer_index = ((), outer_bitmap_index);

                let preimage = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
                    market_key,
                    outer_index,
                };
                let hash = preimage.hash();

                let outer_bitmap = hash.load();

                if !outer_bitmap.is_active() {
                    return None;
                }

                let outer_pos_range = Self::clamped_range::<In>(&range, outer_index);

                let outer_pos_iterator = OuterPosV2::linear_iterator::<In>(outer_pos_range.clone())
                    .filter(move |outer_pos| outer_bitmap.index_active(*outer_pos));

                Some((outer_index, outer_pos_iterator))
            },
        )
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

    fn parent_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<(OuterIndex<Self>, Self)>,
    ) -> impl Iterator<Item = (OuterIndex<Self>, impl Iterator<Item = Self>)>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let start = (((), range.start().0 .0), range.start().0 .1);
        let end = (((), range.end().0 .0), range.end().0 .1);
        let outer_range = start..=end;

        OuterPosV2::parent_iterator::<M, B, Q, In>(market_key, outer_range).flat_map(
            move |((_, outer_bitmap_index), outer_pos_iter)| {
                let range = range.clone();
                outer_pos_iter.flat_map(move |outer_pos| {
                    let outer_index = (outer_bitmap_index, outer_pos);

                    let preimage = BitmapPreimageV2::<M, B, Q, INNER_POS_V2> {
                        market_key,
                        outer_index,
                    };

                    let hash = preimage.hash();

                    let inner_bitmap = hash.load();

                    let inner_pos_range = Self::clamped_range::<In>(&range, outer_index);

                    let inner_pos_iterator =
                        InnerPosV2::linear_iterator::<In>(inner_pos_range.clone())
                            .filter(move |inner_pos| inner_bitmap.index_active(*inner_pos));

                    Some((outer_index, inner_pos_iterator))
                })
            },
        )
    }
}
