use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{
        InnerPosV2, OuterBitmapIndexV2, OuterPosV2, Position, INNER_POS_V2, OUTER_POS_V2,
    },
    state::{bitmap_v2::preimage::BitmapPreimageV2, MarketPreimage, Preimage, SlotKey},
};
use core::ops::RangeInclusive;
pub trait OrderedIndex: Clone + Copy + PartialEq + Default {
    type Prev: OrderedIndex<Prev: Clone + Copy + PartialEq>;

    fn linear_iterator<In>(range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher;

    fn parent_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}

impl OrderedIndex for () {
    type Prev = ();

    fn linear_iterator<In>(_range: RangeInclusive<Self>) -> impl Iterator<Item = Self>
    where
        In: LegMatcher,
    {
        core::iter::empty()
    }

    fn parent_iterator<M, B, Q, In>(
        _market_key: SlotKey<MarketPreimage<M, B, Q>>,
        _range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        core::iter::empty()
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
        _range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        core::iter::empty()
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
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let outer_range = OuterBitmapIndexV2::convert_range(&range);
        OuterBitmapIndexV2::linear_iterator::<In>(outer_range)
            .flat_map(move |outer_bitmap_index| {
                let position = Position::from(outer_bitmap_index);

                let preimage = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
                    market_key,
                    position,
                };
                let outer_bitmap = preimage.hash().load();

                if !outer_bitmap.is_active() {
                    return None;
                }

                let outer_pos_range = Self::effective_range::<In>(&range, position);

                Some(
                    OuterPosV2::linear_iterator::<In>(outer_pos_range)
                        .filter(move |outer_pos| outer_bitmap.index_active(*outer_pos))
                        .map(move |outer_pos| position + outer_pos.into()),
                )
            })
            .flatten()
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
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let start = range.start().complement::<INNER_POS_V2>();
        let end = range.end().complement::<INNER_POS_V2>();
        let outer_range = start..=end;

        OuterPosV2::parent_iterator::<M, B, Q, In>(market_key, outer_range).flat_map(
            move |position| {
                let preimage = BitmapPreimageV2::<M, B, Q, INNER_POS_V2> {
                    market_key,
                    position,
                };
                let inner_bitmap = preimage.hash().load();

                let inner_pos_range = Self::effective_range::<In>(&range, position);

                InnerPosV2::linear_iterator::<In>(inner_pos_range)
                    .filter(move |inner_pos| inner_bitmap.index_active(*inner_pos))
                    .map(move |inner_pos| position + inner_pos.into())
            },
        )
    }
}
