use std::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    matching::bitmap::{
        inner_pos::InnerPos, outer_bitmap_index::OuterBitmapIndex, outer_pos::OuterPos,
    },
    state::{
        bitmap::{
            outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
            Bitmap,
        },
        MarketPreimage, Preimage, SlotKey,
    },
};

pub fn match_order<'a, M, B, Q, In>(
    market_key: &'a SlotKey<MarketPreimage<M, B, Q>>,
    range: RangeInclusive<(OuterBitmapIndex<In>, OuterPos<In>, InnerPos<In>)>,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let outer_bitmap_iter = In::outer_bitmap_index_iter(range.start().0..=range.end().0);

    let final_iter = outer_bitmap_iter
        .filter_map(move |outer_bitmap_index| {
            let preimage = OuterBitmapPreimage {
                market_key: *market_key,
                outer_bitmap_index,
            };
            let outer_bitmap_key = preimage.hash();
            let outer_bitmap = outer_bitmap_key.load();
            let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

            match outer_bitmap_state {
                OuterBitmapState::Active(active_outer_bitmap) => {
                    let on_start = outer_bitmap_index == range.start().0;
                    let on_end = outer_bitmap_index == range.end().0;

                    let child_start = range.start().1.adjust_start(on_start);
                    let child_end = range.end().1.adjust_limit(on_end);
                    let outer_pos_iter = In::outer_pos_iter(child_start..=child_end);

                    return Some((outer_bitmap_index, active_outer_bitmap, outer_pos_iter));
                }
                _ => None,
            }
        })
        .flat_map(
            move |(outer_bitmap_index, outer_bitmap_state, outer_pos_iter)| {
                outer_pos_iter.filter_map(move |outer_pos| {
                    if !outer_bitmap_state.active(outer_pos) {
                        return None;
                    }

                    Some((outer_bitmap_index, outer_pos))
                })
            },
        );

    Ok(())
}
