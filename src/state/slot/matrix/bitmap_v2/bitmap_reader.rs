use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{InnerPosV2, OuterPosV2, Position, PositionRange, INNER_POS_V2, OUTER_POS_V2},
    state::{bitmap_v2::preimage::BitmapPreimageV2, MarketPreimage, Preimage, SlotKey},
};
use core::ops::RangeInclusive;

pub trait BitmapReader: Clone + Copy + PartialEq + Default {
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

impl BitmapReader for OuterPosV2 {
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
        In::outer_bitmap_index_iter_v2(range.clone())
            .filter_map(move |position| {
                let preimage = BitmapPreimageV2::<M, B, Q, OUTER_POS_V2> {
                    market_key,
                    position,
                };
                let outer_bitmap = preimage.hash().load();
                let outer_pos_range = range.effective_range_v2::<In, OUTER_POS_V2>(position);

                outer_bitmap.is_active().then(|| {
                    In::outer_pos_iter_v2(outer_pos_range)
                        .filter(move |outer_pos| outer_bitmap.index_active((*outer_pos).into()))
                        .map(move |outer_pos| position + outer_pos)
                })
            })
            .flatten()
    }
}

impl BitmapReader for InnerPosV2 {
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

                let inner_pos_range = range.effective_range_v2::<In, INNER_POS_V2>(position);

                In::inner_pos_iter_v2(inner_pos_range)
                    .filter(move |inner_pos| inner_bitmap.index_active((*inner_pos).into()))
                    .map(move |inner_pos| position + inner_pos)
            },
        )
    }
}
