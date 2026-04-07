use std::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{OuterBitmapIndexV2, Position},
    state::{
        bitmap::outer_bitmap::{
            outer_bitmap_state::OuterBitmapState, preimage::OuterBitmapPreimage,
        },
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        MarketPreimage, Preimage, SlotKey,
    },
};

pub struct RestingOrderEntryV2<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub position: Position,
    pub resting_order_key: SlotKey<RestingOrderPreimage<M, B, Q>>,
    pub resting_order: RestingOrder,
}

pub fn match_iterator_v2<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    range: RangeInclusive<Position>,
) -> impl Iterator<Item = RestingOrderEntryV2<M, B, Q>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let outer_bitmap_index_start = OuterBitmapIndexV2::from(*range.start());
    let outer_bitmap_index_end = OuterBitmapIndexV2::from(*range.end());

    let mut counter = *range.start();

    loop {
        let outer_bitmap_index = OuterBitmapIndexV2::from(counter);

        let key = OuterBitmapPreimage {
            market_key,
            outer_bitmap_index,
        }
        .hash();
        let state = OuterBitmapState::from(key.load());

        let OuterBitmapState::Active(active) = state else {
            break;
        };

        // TODO start and end of inner loop
    }

    // we are back at separate iterators again
    // build primitive MVP with loop(), then convert to map lazy iterator
    In::position_iter(outer_bitmap_index_start..=outer_bitmap_index_end).filter_map(
        move |position| {
            let outer_bitmap_index = OuterBitmapIndexV2::from(position);

            let key = OuterBitmapPreimage {
                market_key,
                outer_bitmap_index,
            }
            .hash();
            let state = OuterBitmapState::from(key.load());

            let OuterBitmapState::Active(active) = state else {
                return None;
            };

            // What about resetting OuterPos start and end values?
            // Use same logic as before?

            None
        },
    )
}
