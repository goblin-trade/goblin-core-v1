use core::ops::RangeInclusive;

use crate::{
    axis::{
        leg::leg_matcher::LegMatcher, market::market_marker::MarketMarker,
        token::token_marker::TokenMarker,
    },
    quantities::{Position, INNER_POS_V2},
    state::{
        bitmap_v2::{bitmap_reader::BitmapReader, BitmapV2},
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
    BitmapV2::<INNER_POS_V2>::active_iterator::<M, B, Q, In>(market_key, range).map(
        move |position| {
            let preimage = RestingOrderPreimage::<M, B, Q> {
                market_key,
                position,
            };
            let resting_order_key = preimage.hash();
            let resting_order = resting_order_key.load();

            RestingOrderEntryV2 {
                position,
                resting_order_key,
                resting_order,
            }
        },
    )
}
