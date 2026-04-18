use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{
        InnerPosV2, OuterBitmapIndexV2, OuterPosV2, Position, PositionRange, INNER_POS_V2,
        OUTER_POS_V2,
    },
    state::{bitmap_v2::preimage::BitmapPreimageV2, MarketPreimage, Preimage, SlotKey},
};
use core::ops::RangeInclusive;

pub trait OrderedIndex: Clone + Copy + PartialEq + Default {
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher;
}

impl OrderedIndex for OuterPosV2 {
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        let outer_bitmap_index_range = range.map_range(OuterBitmapIndexV2::from);

        In::outer_bitmap_index_iter(outer_bitmap_index_range)
            .filter_map(move |outer_bitmap_index| {
                let position = Position::from(outer_bitmap_index);
                let preimage = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
                    market_key,
                    position,
                };
                let outer_bitmap = preimage.hash().load();

                // TODO update linear iterators to use Position?
                // We convert Position to DerivedPosition, use it to check if active
                // then convert back to Position
                //
                // Should the linear iterator then clear the inner bits?
                // Eg. For start (outer_pos = 1, inner_pos = 1) should we begin iteration
                // from (outer_pos = 1, inner_pos = 0)?
                //
                // Yes. We can't have any inner_pos, because this value is used to build preimage
                let outer_pos_range = range.effective_range::<In, OUTER_POS_V2>(position);

                outer_bitmap.is_active().then(|| {
                    In::outer_pos_iter(outer_pos_range)
                        .filter(move |outer_pos| outer_bitmap.index_active(*outer_pos))
                        .map(move |outer_pos| position + outer_pos.into())
                })
            })
            .flatten()
    }
}

impl OrderedIndex for InnerPosV2 {
    fn active_iterator<M, B, Q, In>(
        market_key: SlotKey<MarketPreimage<M, B, Q>>,
        range: RangeInclusive<Position>,
    ) -> impl Iterator<Item = Position>
    where
        M: MarketMarker,
        B: TokenMarker,
        Q: TokenMarker,
        In: LegMatcher,
    {
        // outer iterator ignores inner bits in range endpoints — no complement needed
        OuterPosV2::active_iterator::<M, B, Q, In>(market_key, range.clone()).flat_map(
            move |position| {
                let preimage = BitmapPreimageV2::<M, B, Q, INNER_POS_V2> {
                    market_key,
                    position,
                };
                let inner_bitmap = preimage.hash().load();

                let inner_pos_range = range.effective_range::<In, INNER_POS_V2>(position);

                In::inner_pos_iter(inner_pos_range)
                    .filter(move |inner_pos| inner_bitmap.index_active(*inner_pos))
                    .map(move |inner_pos| position + inner_pos.into())
            },
        )
    }
}
