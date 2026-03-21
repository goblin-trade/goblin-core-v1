use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    matching::{
        bitmap::FullCoordinates,
        match_iterator::{inner_bitmap_iter, outer_bitmap_iter, resting_order_iter},
    },
    state::{
        resting_order::{preimage::RestingOrderPreimage, RestingOrder},
        MarketPreimage, SlotKey,
    },
};

pub struct RestingOrderEntry<M, B, Q>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
{
    pub full_coordinates: FullCoordinates,
    pub resting_order_key: SlotKey<RestingOrderPreimage<M, B, Q>>,
    pub resting_order: RestingOrder<M, B, Q>,
}

/// Match iterator generates consecutive resting orders for a side
/// beginning from centre of the book
pub fn match_iterator<M, B, Q, In>(
    market_key: SlotKey<MarketPreimage<M, B, Q>>,
    start: FullCoordinates,
    end: FullCoordinates,
) -> impl Iterator<Item = RestingOrderEntry<M, B, Q>>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    outer_bitmap_iter::<M, B, Q, In>(market_key, start, end)
        .flat_map(move |outer| inner_bitmap_iter::<M, B, Q, In>(outer, start, end))
        .flat_map(resting_order_iter::<M, B, Q, In>)
}
