use core::ops::RangeInclusive;

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
            inner_bitmap::preimage::InnerBitmapPreimage,
            outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
            Bitmap,
        },
        resting_order::preimage::RestingOrderPreimage,
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
    let (outer_bitmap_index_start, outer_pos_start, inner_pos_start) = *range.start();
    let (outer_bitmap_index_end, outer_pos_end, inner_pos_end) = *range.end();

    let outer_bitmap_iter =
        In::outer_bitmap_index_iter(outer_bitmap_index_start..=outer_bitmap_index_end);

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
                    let on_start = outer_bitmap_index == outer_bitmap_index_start;
                    let on_end = outer_bitmap_index == outer_bitmap_index_end;

                    let child_start = outer_pos_start.adjust_start(on_start);
                    let child_end = outer_pos_end.adjust_end(on_end);
                    let outer_pos_iter = In::outer_pos_iter(child_start..=child_end);

                    return Some((
                        outer_bitmap_index,
                        outer_bitmap_key,
                        active_outer_bitmap,
                        outer_pos_iter,
                    ));
                }
                _ => None,
            }
        })
        .flat_map(
            move |(outer_bitmap_index, outer_bitmap_key, outer_bitmap_state, outer_pos_iter)| {
                outer_pos_iter.filter_map(move |outer_pos| {
                    if !outer_bitmap_state.active(outer_pos) {
                        return None;
                    }

                    let preimage = InnerBitmapPreimage {
                        outer_bitmap_key,
                        outer_pos,
                    };
                    let inner_bitmap_key = preimage.hash();
                    let inner_bitmap = inner_bitmap_key.load();

                    let child_start = inner_pos_start.adjust_start(
                        (outer_bitmap_index, outer_pos)
                            == (outer_bitmap_index_start, outer_pos_start),
                    );

                    let child_end = inner_pos_end.adjust_end(
                        (outer_bitmap_index, outer_pos) == (outer_bitmap_index_end, outer_pos_end),
                    );

                    let inner_pos_iter = In::inner_pos_iter(child_start..=child_end);

                    Some((
                        outer_bitmap_index,
                        outer_pos,
                        inner_bitmap_key,
                        inner_bitmap,
                        inner_pos_iter,
                    ))
                })
            },
        )
        .flat_map(
            move |(
                outer_bitmap_index,
                outer_pos,
                inner_bitmap_key,
                inner_bitmap,
                inner_pos_iter,
            )| {
                inner_pos_iter.filter_map(move |inner_pos| {
                    if !inner_bitmap.active(inner_pos) {
                        return None;
                    }

                    let preimage = RestingOrderPreimage {
                        inner_bitmap_key,
                        inner_pos,
                    };
                    let resting_order_key = preimage.hash();
                    let resting_order = resting_order_key.load();

                    Some((
                        outer_bitmap_index,
                        outer_pos,
                        inner_pos,
                        resting_order_key,
                        resting_order,
                    ))
                })
            },
        );

    Ok(())
}
