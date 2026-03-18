use core::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::bitmap::FullCoordinates,
    state::{
        bitmap::{
            inner_bitmap::preimage::InnerBitmapPreimage,
            outer_bitmap::{outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage},
            Bitmap,
        },
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        MarketPreimage, Preimage, SlotKey,
    },
};

pub fn flat_iterator<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    range: RangeInclusive<FullCoordinates<In>>,
) -> impl Iterator<
    Item = (
        FullCoordinates<In>,
        SlotKey<RestingOrderPreimage<M, B, Q, In>>,
        RestingOrder<M, B, Q>,
    ),
>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let start = *range.start();
    let end = *range.end();

    let outer_bitmap_iter =
        In::outer_bitmap_index_iter(start.outer_bitmap_index..=end.outer_bitmap_index);

    let final_iter = outer_bitmap_iter
        .filter_map(move |outer_bitmap_index| {
            let preimage = OuterBitmapPreimage {
                market_key,
                outer_bitmap_index,
            };
            let outer_bitmap_key = preimage.hash();
            let outer_bitmap = outer_bitmap_key.load();
            let outer_bitmap_state = OuterBitmapState::from(outer_bitmap);

            match outer_bitmap_state {
                OuterBitmapState::Active(active_outer_bitmap) => {
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

                    let child_start = if (outer_bitmap_index, outer_pos)
                        == (start.outer_bitmap_index, start.outer_pos)
                    {
                        start.inner_pos
                    } else {
                        In::start_value()
                    };
                    let child_end = if (outer_bitmap_index, outer_pos)
                        == (end.outer_bitmap_index, end.outer_pos)
                    {
                        end.inner_pos
                    } else {
                        In::end_value()
                    };

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
                        FullCoordinates {
                            outer_bitmap_index,
                            outer_pos,
                            inner_pos,
                        },
                        resting_order_key,
                        resting_order,
                    ))
                })
            },
        );

    final_iter
}
