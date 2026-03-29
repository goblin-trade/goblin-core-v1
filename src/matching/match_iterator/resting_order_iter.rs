use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        bitmap::FullCoordinates,
        match_iterator::{inner_bitmap_iter::InnerBitmapEntry, RestingOrderEntry},
    },
    state::{bitmap::Bitmap, resting_order::preimage::RestingOrderPreimage, Preimage},
};

pub fn resting_order_iter<M, B, Q, In>(
    InnerBitmapEntry {
        outer_bitmap_index,
        outer_pos,
        inner_bitmap_key,
        inner_bitmap,
        child_range,
    }: InnerBitmapEntry<M, B, Q>,
) -> impl Iterator<Item = RestingOrderEntry<M, B, Q>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    In::inner_pos_iter(child_range).filter_map(move |inner_pos| {
        if !inner_bitmap.pos_active(inner_pos) {
            return None;
        }

        let key = RestingOrderPreimage {
            inner_bitmap_key,
            inner_pos,
        }
        .hash();
        let resting_order = key.load();

        Some(RestingOrderEntry {
            full_coordinates: FullCoordinates {
                outer_bitmap_index,
                outer_pos,
                inner_pos,
            },
            resting_order_key: key,
            resting_order,
        })
    })
}
