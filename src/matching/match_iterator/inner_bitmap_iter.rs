use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        bitmap::{
            inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
            FullCoordinates,
        },
        match_iterator::outer_bitmap_iter::OuterBitmapEntry,
    },
    state::{
        bitmap::{
            inner_bitmap::{preimage::InnerBitmapPreimage, InnerBitmap},
            Bitmap,
        },
        Preimage, SlotKey,
    },
};
use core::ops::RangeInclusive;

pub struct InnerBitmapEntry<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub outer_bitmap_index: OuterBitmapIndex,
    pub outer_pos: OuterPos,
    pub inner_bitmap_key: SlotKey<InnerBitmapPreimage<M, B, Q>>,
    pub inner_bitmap: InnerBitmap,
    pub child_range: RangeInclusive<InnerPos>,
}

pub fn inner_bitmap_iter<M, B, Q, In>(
    OuterBitmapEntry {
        outer_bitmap_index,
        outer_bitmap_key,
        active_outer_bitmap,
        child_range,
    }: OuterBitmapEntry<M, B, Q>,
    start: FullCoordinates,
    end: FullCoordinates,
) -> impl Iterator<Item = InnerBitmapEntry<M, B, Q>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    In::outer_pos_iter(child_range).filter_map(move |outer_pos| {
        if !active_outer_bitmap.active(outer_pos) {
            return None;
        }

        let key = InnerBitmapPreimage {
            outer_bitmap_key,
            outer_pos,
        }
        .hash();
        let bitmap = key.load();

        let child_start =
            if (outer_bitmap_index, outer_pos) == (start.outer_bitmap_index, start.outer_pos) {
                start.inner_pos
            } else {
                In::start_value()
            };
        let child_end =
            if (outer_bitmap_index, outer_pos) == (end.outer_bitmap_index, end.outer_pos) {
                end.inner_pos
            } else {
                In::end_value()
            };

        Some(InnerBitmapEntry {
            outer_bitmap_index,
            outer_pos,
            inner_bitmap_key: key,
            inner_bitmap: bitmap,
            child_range: child_start..=child_end,
        })
    })
}
