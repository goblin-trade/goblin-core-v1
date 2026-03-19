use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        bitmap::FullCoordinates,
        flat_iterator::{
            inner_bitmap_iter, outer_bitmap_iter,
            resting_order_iter::{resting_order_iter, RestingOrderEntry},
        },
    },
    state::{MarketPreimage, SlotKey},
};

pub fn flat_iterator<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    start: FullCoordinates<In>,
    end: FullCoordinates<In>,
) -> impl Iterator<Item = RestingOrderEntry<M, B, Q, In>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    outer_bitmap_iter(market_key, start, end)
        .flat_map(move |outer| inner_bitmap_iter(outer, start, end))
        .flat_map(resting_order_iter)
}
