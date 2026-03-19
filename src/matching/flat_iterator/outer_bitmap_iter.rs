use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::{
        outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos, FullCoordinates,
    },
    state::{
        bitmap::outer_bitmap::{
            active_outer_bitmap::ActiveOuterBitmap, outer_bitmap_state::OuterBitmapState,
            preimage::OuterBitmapPreimage,
        },
        MarketPreimage, Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

pub struct OuterBitmapEntry<M, B, Q, In>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    pub outer_bitmap_index: OuterBitmapIndex<In>,
    pub outer_bitmap_key: SlotKey<OuterBitmapPreimage<M, B, Q, In>>,
    pub active_outer_bitmap: ActiveOuterBitmap<M, B, Q>,
    pub child_range: RangeInclusive<OuterPos<In>>,
}

pub fn outer_bitmap_iter<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    start: FullCoordinates<In>,
    end: FullCoordinates<In>,
) -> impl Iterator<Item = OuterBitmapEntry<M, B, Q, In>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    In::outer_bitmap_index_iter(start.outer_bitmap_index..=end.outer_bitmap_index).filter_map(
        move |outer_bitmap_index| {
            let key = OuterBitmapPreimage {
                market_key,
                outer_bitmap_index,
            }
            .hash();
            let state = OuterBitmapState::from(key.load());

            let OuterBitmapState::Active(active) = state else {
                return None;
            };

            // Clamp child range to the level-specific bounds
            let child_start = if outer_bitmap_index == start.outer_bitmap_index {
                start.outer_pos
            } else {
                In::start_value()
            };
            let child_end = if outer_bitmap_index == end.outer_bitmap_index {
                end.outer_pos
            } else {
                In::end_value()
            };

            Some(OuterBitmapEntry {
                outer_bitmap_index,
                outer_bitmap_key: key,
                active_outer_bitmap: active,
                child_range: child_start..=child_end,
            })
        },
    )
}
